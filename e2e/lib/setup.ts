import { Account, connect, Contract } from "near-api-js";
import { getConfig } from "./config";
import { deployContract, initContract } from "./deploy";
import { createFSKeyStore, getAccount } from "./store";
import * as ITO from '../../ito';
import BN from "bn.js";

export type ITOContract = Contract & ITO.Contract;

export async function setup(): Promise<[Account, ...(Contract & ITO.Contract)[]]> {
    const env = process.env['ENV'] ?? useSandbox();

    const near = await connect({
        keyStore: createFSKeyStore(),
        ...getConfig(env!),
    });

    const [ito, alreadyDeployed] = await deployContract(near, 'ito', 'target/wasm32-unknown-unknown/release/ito_contract.wasm');

    async function contract(name: string): Promise<Contract & ITO.Contract> {
        const account = await getAccount(name, near);
        return new Contract(account, ito.accountId, ITO.ContractMethods) as any;
    }

    const accounts = await Promise.all(['owner', 'alice', 'bob', 'carol'].map(name => contract(name)));
    const owner = accounts[0];

    if (!alreadyDeployed) {
        await initContract<ITO.Contract, keyof ITO.Contract>(owner,
            {
                func: 'new_default_meta',
                args: {
                    owner_id: owner.account.accountId,
                    treasury_id: owner.account.accountId
                },
            }
        );
    }

    return [ito, ...accounts];
}

function useSandbox() {
    console.info('Using ENV `sandbox`');

    return 'sandbox';
}

declare global {
    interface String {
        bn(this: string): BN;
    }
}

String.prototype.bn = function (this: string) { return new BN(this); };
