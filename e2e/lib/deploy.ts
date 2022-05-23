import BN from 'bn.js';
import * as bs58 from 'bs58';
import { readFileSync } from 'fs';
import { sha256 } from 'js-sha256';
import type { Account, Contract } from 'near-api-js';
import { Near } from 'near-api-js';
import { AccountBalance, FunctionCallOptions } from 'near-api-js/lib/account';
import { AccountView } from 'near-api-js/lib/providers/provider';
import { formatNearAmount } from 'near-api-js/lib/utils/format';
import { basename } from 'path';
import { getAccount } from './store';

export const MAX_GAS_ALLOWED = new BN(300000000000000);

const logAmount = (value: string) => formatNearAmount(value, 4);

export const getState = async (account: Account, prefix: string): Promise<AccountView & AccountBalance> => {
    const state = await account.state();
    const balance = await account.getAccountBalance();

    if (!new BN(balance.total).eq(new BN(balance.stateStaked).add(new BN(balance.available)))) {
        console.log('Total neq staked+available');
    }

    const isContract = state.code_hash === '11111111111111111111111111111111' ? '\u261e' : '\u270e';
    console.log(`${isContract} ${prefix}: Ⓝ S${logAmount(balance.stateStaked)}+A${logAmount(balance.available)}`);

    return { ...state, ...balance };
};

export async function deployContract(near: Near, contractPrefix: string, wasmPath: string): Promise<[Account, boolean]> {
    const contractAccount = await getAccount(contractPrefix, near);
    const contract = await getState(contractAccount, contractPrefix);

    const wasmData = readFileSync(wasmPath);
    const wasmHash = sha256.array(wasmData);
    const wasmBase64 = bs58.encode(Buffer.from(wasmHash));

    console.log(`Contract ${basename(wasmPath)} sha256/base58:${wasmBase64}`);

    let alreadyDeployed;
    if (contract.code_hash !== wasmBase64) {
        console.log('  ... deploying\n');

        await contractAccount.deployContract(wasmData);
        await getState(contractAccount, contractPrefix);
        alreadyDeployed = false;
    } else {
        console.log('  ... up to date\n');
        alreadyDeployed = true;
    }

    return [contractAccount, alreadyDeployed];
};

export async function initContract<T, S extends keyof T & string>(
    contractAccount: Contract,
    init: { func: S; args: T[S] }
) {
    const options: FunctionCallOptions = {
        contractId: contractAccount.contractId,
        methodName: init.func,
        args: init.args as any,
        gas: MAX_GAS_ALLOWED,
        attachedDeposit: new BN(0),
    };

    try {
        await contractAccount.account.functionCall(options);
    } catch (err) {
        const regex = /The contract has already been initialized/;
        const msg = (err as any)?.kind?.ExecutionError;
        if (!regex.exec(msg)) {
            throw err;
        }

        console.log(regex.source);
    }
}
