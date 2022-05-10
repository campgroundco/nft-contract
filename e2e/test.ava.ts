import test from 'ava';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';

test('near amount check', t => {
    t.is(formatNearAmount('1'), `0.${'0'.repeat(23)}1`);
    t.is(parseNearAmount('1'), '1' + '0'.repeat(24));
    t.is(parseNearAmount('0.1'), '1' + '0'.repeat(23));
});

test('wip', async t => {
    const bar = Promise.resolve('bar');
    t.is(await bar, 'bar');
});
