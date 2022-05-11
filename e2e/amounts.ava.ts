import test from 'ava';
import BN from 'bn.js';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';

test('parse/format NEAR amount checks', t => {
    t.is(formatNearAmount('1'), `0.${'0'.repeat(23)}1`);
    t.is(parseNearAmount('1'), '1' + '0'.repeat(24));
    t.is(parseNearAmount('0.1'), '1' + '0'.repeat(23));

    t.is(formatNearAmount('10000000000000000000000000'), '10');
    t.is(formatNearAmount('12114570000000000000000000'), '12.11457');

    t.is(formatNearAmount(new BN(10).pow(new BN(25)).toString()), '10');
});
