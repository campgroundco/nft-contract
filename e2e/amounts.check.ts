import BN from 'bn.js';
import { expect } from 'chai';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';

describe('amounts', () => {

    it.only('should parse/format NEAR amounts', () => {
        expect(formatNearAmount('1')).to.be.equal(`0.${'0'.repeat(23)}1`);
        expect(parseNearAmount('1')).to.be.equal('1' + '0'.repeat(24));
        expect(parseNearAmount('0.1')).to.be.equal('1' + '0'.repeat(23));

        expect(formatNearAmount('10000000000000000000000000')).to.be.equal('10');
        expect(formatNearAmount('12114570000000000000000000')).to.be.equal('12.11457');

        expect(formatNearAmount(new BN(10).pow(new BN(25)).toString())).to.be.equal('10');
    });

    it.only('should ITO contract balance create_series', () => {
        const attachedDeposit = '5780000000000000000000';

        const itoBalance = {
            total: '500030630918771186000000000',
            stateStaked: '3701770000000000000000000',
            available: '496328860918771186000000000'
        };
        const aliceBalance = {
            total: '499904822782897427000000000',
            stateStaked: '1820000000000000000000',
            available: '499903002782897427000000000'
        };
        const itoPostBalance = {
            total: '500035913822870552000000000',
            stateStaked: '3706410000000000000000000',
            available: '496329503822870552000000000'
        };
        const alicePostBalance = {
            total: '499892959765331280000000000',
            stateStaked: '1820000000000000000000',
            available: '499891139765331280000000000'
        };
        const receipts = [
            { executor_id: 'ito', gas_burnt: 4571424334101, tokens_burnt: '4571424334101000000000' },
            { executor_id: 'alice', gas_burnt: 223182562500, tokens_burnt: '223182562500000000000' },
        ];
        const outcome = { executor_id: 'alice', gas_burnt: 2428410669546, tokens_burnt: '2428410669546000000000' };

        const totalBalance = itoBalance.total.bn().add(aliceBalance.total.bn())
        const totalPostBalance = itoPostBalance.total.bn().add(alicePostBalance.total.bn());
        const gasUsed = totalBalance.sub(totalPostBalance);

        const itoEarns = itoPostBalance.total.bn().sub(itoBalance.total.bn());
        const storageUsed = itoPostBalance.stateStaked.bn().sub(itoBalance.stateStaked.bn());
        const storageUsedExcess = itoPostBalance.available.bn().sub(itoBalance.available.bn());

        const alicePays = aliceBalance.total.bn().sub(alicePostBalance.total.bn());
        const alicePaysExceptGas = alicePays.sub(gasUsed);

        const a = receipts[0].tokens_burnt.bn();
        const b = receipts[1].tokens_burnt.bn();
        const aliceBurnt = outcome.tokens_burnt.bn();

        // console.log(formatNearAmount(a.toString()));
        // console.log(formatNearAmount(b.toString()));
        // console.log(formatNearAmount(c.toString()));
        console.log(formatNearAmount(aliceBalance.total.bn().sub(aliceBurnt).sub(alicePostBalance.total.bn()).toString()));

        console.log('gas used', formatNearAmount(gasUsed.toString()));
        console.log('storage used', formatNearAmount(storageUsed.toString()));
        console.log('storage used excess', formatNearAmount(storageUsedExcess.toString()));
        console.log('storage used+excess', formatNearAmount(storageUsedExcess.add(storageUsed).toString()));
        console.log('attached deposit', formatNearAmount(attachedDeposit));
        console.log('alice pays', formatNearAmount(alicePays.toString()));
        console.log('ito earns', formatNearAmount(itoEarns.toString()));
        console.log('alice except gas', formatNearAmount(alicePaysExceptGas.toString()));

        expect(itoPostBalance.total.bn().gt(itoBalance.total.bn())).to.be.true;
        expect(itoPostBalance.stateStaked.bn().gt(itoBalance.stateStaked.bn())).to.be.true;
        expect(itoPostBalance.available.bn().gt(itoBalance.available.bn())).to.be.true;
        expect(attachedDeposit.bn().gt(storageUsed)).to.be.true;

        expect(alicePostBalance.total.bn().lt(aliceBalance.total.bn())).to.be.true;
        expect(alicePostBalance.stateStaked).to.be.equal(aliceBalance.stateStaked);
        expect(alicePostBalance.available.bn().lt(aliceBalance.available.bn())).to.be.true;


    });

    it.only('should check after but nft', () => {
        const itoBalance = {
            total: '499979692556007628000000000',
            stateStaked: '3656910000000000000000000',
            available: '496322782556007628000000000'
        };
        const bobBalance = {
            total: '500000000000000000000000000',
            stateStaked: '1820000000000000000000',
            available: '499998180000000000000000000'
        };
        const carolBalance = {
            total: '499987477763697523000000000',
            stateStaked: '1820000000000000000000',
            available: '499985657763697523000000000'
        };

        const campgroundPercentage = 5;
        const accountPercentage = 100 - campgroundPercentage;
        const price = '5000000000000000000000000';
        const attachedDeposit = '5007940000000001000000000';

        const itoPostBalance = {
            total: '499988187763004240000000000',
            stateStaked: '3664510000000000000000000',
            available: '496323677763004240000000000'
        }
        const bobPostBalance = {
            total: '494983890193466076000000000',
            stateStaked: '1820000000000000000000',
            available: '494982070193466076000000000'
        };
        const carolPostBalance = {
            total: '504737477763697523000000000',
            stateStaked: '1820000000000000000000',
            available: '504735657763697523000000000'
        };

        const carolEarns = carolPostBalance.total.bn().sub(carolBalance.total.bn());
        expect(formatNearAmount(carolEarns.toString())).to.be.equal(formatNearAmount(price.bn().muln(accountPercentage).divn(100).toString()));

    });
});