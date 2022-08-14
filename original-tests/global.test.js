const WiseToken = artifacts.require("WiseToken");
const LiquidityTransformer = artifacts.require("LiquidityTransformer");
const RefundSponsor = artifacts.require("RefundSponsor");
const catchRevert = require("./exceptionsHelpers.js").catchRevert;

require("./utils");
const BN = web3.utils.BN;

// TESTING PARAMETERS
const SECONDS_IN_DAY = 30;
const HALF_ETH = web3.utils.toWei("0.5");
const HALF_ETH_WITH_BONUS = web3.utils.toWei("0.55");
const ONE_ETH = web3.utils.toWei("1");
const TWO_ETH = web3.utils.toWei("2");
const FIFTY_ETH = web3.utils.toWei("50");
const FIVE_ETH = web3.utils.toWei("5");
const STATIC_SUPPLY = web3.utils.toWei("5000000");
const TWO_ETH_WITH_BONUS = web3.utils.toWei("2.2"); // +10% bonus
const DAILY_BONUS = 13698630136986302
const INVESTMENT_DAYS = 50;
const PRE_LAUNCH_DAYS = 0;
const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

const getLastEvent = async (eventName, instance) => {
    const events = await instance.getPastEvents(eventName, {
        fromBlock: 0,
        toBlock: "latest",
    });
    return events.pop().returnValues;
};

contract("LiquidityTransformer", ([owner, user1, user2, random]) => {

    let token;
    let launchTime;

    describe("Ability to generate tokens and forwarding liquidity", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);
        });

        it("should pre-calculate referrals and forward liquidity", async () => {

            // try to forward liquidity should fail
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: ongoing investment phase"
            );

            await lt.reserveWise([49], user2, { from: user1, value: ONE_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 50) * SECONDS_IN_DAY);
            const ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), 50);

            // still be able to reserve on day 50
            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: ONE_ETH });

            dailyTotalInvestmentDay50 = await lt.dailyTotalInvestment(50);
            assert.equal(dailyTotalInvestmentDay50.toString(), ONE_ETH.toString());

            //not be able to reserve for day 49 and below
            await catchRevert(
                lt.reserveWise([49], user2, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            // try to forward liquidity should fail
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: ongoing investment phase"
            );

            // fastforward + 1day = to day 51
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            //not be able to reserve for day 50
            await catchRevert(
                lt.reserveWise([50], user2, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            //not be able to reserve for any days distributed
            await catchRevert(
                lt.reserveWise([50], user2, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            // try to forward liquidity should still fail since need to generate supply first
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: must generate supply for all days"
            );

            let dailyTotalSupplyOnDay49 = await lt.dailyTotalSupply(49);
            assert.equal(dailyTotalSupplyOnDay49.toString(), "0");
            // check day supply after generating

            await lt.generateSupply(49, { from: user1 });
            dailyTotalSupplyOnDay49 = await lt.dailyTotalSupply(49);

            assert.equal(dailyTotalSupplyOnDay49.toString(), STATIC_SUPPLY);

            // try to forward liquidity should still fail since need to generate supply for last day
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: must generate supply for all days"
            );

            let dailyTotalSupplyOnDay50 = await lt.dailyTotalSupply(50);
            assert.equal(dailyTotalSupplyOnDay50.toString(), "0");

            await lt.generateSupply(50, { from: user1 });

            dailyTotalSupplyOnDay50 = await lt.dailyTotalSupply(50);
            assert.equal(dailyTotalSupplyOnDay50.toString(), STATIC_SUPPLY);

            // try to forward liquidity should still fail since need to pre-calc referrers
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: must prepare all referrals"
            );

            await catchRevert(
                lt.prepareReferralBonuses(1, 0, { from: user1 }),
                "revert WISE: incorrect referral batch"
            );

            let referralAmountA = await lt.referralAmount(user2);
            assert.equal(referralAmountA.toString(), ONE_ETH);

            await lt.prepareReferralBonuses(0, 1, { from: user1 });

            referralAmountA = await lt.referralAmount(user2);
            assert.equal(referralAmountA.toString(), "0");

            referralTokensA = await lt.referralTokens(user2);

            // finally try to forward liquidity
            await lt.forwardLiquidity({ from: random });

            const data = await getLastEvent(
                "UniSwapResult",
                lt
            );

            await lt.$getMyTokens({ from: user1 });
            const { value } = await getLastEvent(
                "Transfer",
                token
            );

            const balanceOfUser = await token.balanceOf(user1);
            assert.equal(balanceOfUser.toString(), value.toString());
        });
    });

    describe("Regular Shares /  Referrer Shares", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);

            await lt.reserveWise([49], user2, { from: user1, value: ONE_ETH });
            await lt.reserveWise([50], random, { from: user2, value: ONE_ETH });
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });
            await lt.prepareReferralBonuses(0, 2, { from: user1 });
            await lt.forwardLiquidity({ from: random });

            await lt.$getMyTokens({ from: user1 });
            await lt.$getMyTokens({ from: user2 });
            await lt.$getMyTokens({ from: random });

            await token.manualDailySnapshot({ from: user2, gas: 12000000 });
        });

        it("should allow ladder staking", async () => {

            await token.createStake(web3.utils.toWei("100"), 10, ZERO_ADDRESS, { from: user1 });
            const stakeA = await getLastEvent("StakeStart", token);
            await token.createStake(web3.utils.toWei("300"), 50, ZERO_ADDRESS, { from: user2 });
            const stakeB = await getLastEvent("StakeStart", token);
            await token.createStake(web3.utils.toWei("50"), 75, ZERO_ADDRESS, { from: user1 });
            const stakeC = await getLastEvent("StakeStart", token);

            await advanceTimeAndBlock(25 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            await token.createStake(web3.utils.toWei("100"), 100, ZERO_ADDRESS, { from: user2 });
            const stakeD = await getLastEvent("StakeStart", token);
            await token.createStake(web3.utils.toWei("200"), 50, ZERO_ADDRESS, { from: user1 });
            const stakeE = await getLastEvent("StakeStart", token);

            await token.endStake(stakeA.stakeID, { from: user1 });

            await advanceTimeAndBlock(25 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });
            await token.endStake(stakeB.stakeID, { from: user2 });

            await advanceTimeAndBlock(25 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });
            await token.endStake(stakeE.stakeID, { from: user1 });

            await advanceTimeAndBlock(25 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            await token.endStake(stakeD.stakeID, { from: user2 });
            await token.endStake(stakeC.stakeID, { from: user1 });
        });

        it("should have correct balance after minting", async () => {
            const balanceOfUser1 = await token.balanceOf(user1);
            const balanceOfUser2 = await token.balanceOf(user2);
            const balanceOfUser3 = await token.balanceOf(random);

            // console.log(balanceOfUser1.toString()); // 5000000000000000000000000
            // console.log(balanceOfUser2.toString()); // 5250000000000000000000000
            // console.log(balanceOfUser3.toString()); // 250000000000000000000000

            assert.equal(balanceOfUser1.toString(), '5000000000000000000000000');
            assert.equal(balanceOfUser2.toString(), '5250000000000000000000000');
            assert.equal(balanceOfUser3.toString(), '250000000000000000000000');
        });

        it("should use less gas to create stake", async () => {
            // create stake
            // currently 230k without referrer
            // currnetly 420k with referrer
            const _lockDays = 365;
            await token.createStake(web3.utils.toWei("100"), _lockDays, ZERO_ADDRESS, { from: user1 });
        });

        /*it("should generate correct amount of regular/referrer shares (365 days)", async () => {
            // create stake
            const _lockDays = 365;

            await token.createStake(web3.utils.toWei("100"), _lockDays, user2, { from: user1 });
            stakeId = await token.myLatestStakeID({ from: user1 });
            referralId = await token.myLatestReferralID({ from: user2 });

            const {
                stakesShares,
                referrerShares
            } = await token.stakes(user1, stakeId);

            const {
                staker,
                stakeID,
                isActive,
            } = await token.referrerLinks(user2, referralId);

            assert.equal(staker, user1);
            assert.equal(stakeID, stakeId);
            assert.equal(isActive, true);

            // console.log(stakesShares.toString()); // 1150000000000000000000
            // console.log(referrerShares.toString()); // 1050000000000000000000

            assert.equal(stakesShares.toString(), '1150000000000000000000');
            assert.equal(referrerShares.toString(), '1050000000000000000000');

            // one day passes
            await advanceTimeAndBlock(SECONDS_IN_DAY);

            // one day passes
            await advanceTimeAndBlock(SECONDS_IN_DAY);

            // 365 days passes
            for (let i = 0; i <= _lockDays; i++) {
                await advanceTimeAndBlock(SECONDS_IN_DAY);
                await token.manualDailySnapshot({ from: user2 });
            }

            await token.endStake(stakeId, { from: user1, gas: 12000000 });
        });*/

        it("should change sharePrice after stake is closed with interest higher than current price", async () => {
            const _lockDays = 20;
            const before = await token.globals();
            await token.createStake(web3.utils.toWei("100"), _lockDays, ZERO_ADDRESS, { from: user1 });
            const stakeA = await getLastEvent("StakeStart", token);
            await advanceTimeAndBlock(25 * SECONDS_IN_DAY);
            await token.endStake(stakeA.stakeID, { from: user1 });
            const after = await token.globals();
            assert.equal(after.sharePrice > before.sharePrice, true);
        });
    });

    describe("Ability to refund gas expenses: Liquidity Transformer", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
            sponsorAddress = await lt.REFUND_SPONSOR();
            sp = await RefundSponsor.at(sponsorAddress);
            sp.sendTransaction({from: user1, value: ONE_ETH})
            await sp.setSponsoredContract(lt.address);
            await token.setLiquidityTransfomer(lt.address);
            await sp.flush(); // flush values;
            // console.log(sponsorAddress, 'refundSponsor');
        });

        it("should store gas amount for refund", async () => {
            await lt.reserveWise([49], user2, { from: user1, value: ONE_ETH, gasPrice: 20000000000 });
            const amount = await sp.getRefundAmount(user1);
            // console.log(amount.toString());
            // console.log(user1);
            assert.equal(amount > 0, true);
        });

        it("should be able to flush stored gas amounts", async () => {
            await lt.reserveWise([49], user2, { from: user1, value: ONE_ETH, gasPrice: 20000000000 });
            const amountBefore = await sp.getRefundAmount(user1);
            assert.equal(amountBefore > 0, true);
            const nonceBefore = await sp.flushNonce();

            await sp.flush(); // flush values;

            const amountAfter = await sp.getRefundAmount(user1);
            const nonceAfter = await sp.flushNonce();
            // console.log(amountAfter.toString());
            assert.equal(amountAfter.toString(), '0');
            assert.equal(nonceAfter.toNumber(), nonceBefore.toNumber() + 1);
            await lt.reserveWise([50], user2, { from: user1, value: ONE_ETH, gasPrice: 20000000000 });
            const amount = await sp.getRefundAmount(user1);
            assert.equal(amount > 0, true);
        });

        it("should track gas refunds", async () => {
            const res = await lt.reserveWise([50], user2, { from: user1, value: ONE_ETH, gasPrice: 20000000000 });
            const amountRefunded = await sp.getRefundAmount(user1);
            // console.log(amountRefunded.toString(), 'refundAmount');
            // console.log(res.receipt.gasUsed, 'GAS-USED');
            assert.equal(amountRefunded > 0, true);
            assert.equal(amountRefunded < res.receipt.gasUsed * 20000000000, true);
            assert.equal(amountRefunded * 100 / 70 < res.receipt.gasUsed * 20000000000, true);
            // console.log(res.receipt.gasUsed * 20000000000, 'GAS-PRICE');
            // console.log(res);
        });

        it("should allow to request gas refund", async () => {
            await lt.reserveWise([50], user2, { from: user1, value: ONE_ETH, gasPrice: 20000000000 });
            const amountRefunded = await sp.getRefundAmount(user1);
            await sp.requestGasRefund({from: user1});
            const { refundedTo, amount } = await getLastEvent("RefundIssued", sp);
            assert.equal(refundedTo, user1);
            // console.log(amount, 'refunded');
            assert.equal(amount > 0, true);
            assert.equal(amount.toString(), amountRefunded.toString());
        });
    });

    describe("Ability to scrape interest", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);

            await lt.reserveWise([49], user2, { from: user1, value: ONE_ETH });
            await lt.reserveWise([50], random, { from: user2, value: ONE_ETH });
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });
            await lt.prepareReferralBonuses(0, 2, { from: user1 });
            await lt.forwardLiquidity({ from: random });

            await lt.$getMyTokens({ from: user1 });
            await lt.$getMyTokens({ from: user2 });
            await lt.$getMyTokens({ from: random });

            await token.manualDailySnapshot({ from: user2, gas: 12000000 });
        });

        it("should NOT be able to scrape interest (from immature stake) if penalty is higher than current shares: scenarioA", async () => {

            // -------------------
            // scenario A:
            // 1) when scrape amount is higher than initial stake
            // createStake
            await token.createStake(web3.utils.toWei("100"), 10, ZERO_ADDRESS, { from: user1 });
            const stake = await getLastEvent("StakeStart", token);

            // fast-forward 5 days
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user2 });

            let latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const stakeDataBefore = await token.checkStakeByID(user1, latestStakeID);
            // console.log(stakeDataBefore.rewardAmount, 'reward before');

            assert.equal(
                parseInt(stakeDataBefore.stakedAmount.toString()) <
                parseInt(stakeDataBefore.rewardAmount.toString()), true
            );

            await catchRevert(
                token.scrapeInterest(
                    stake.stakeID,
                    0,
                    { from: user1 }
                )
            );
        });

        it("should be able to scrape interest (from immature stake) with penalty: scenarioB", async () => {

            // -------------------
            // scenario B:
            // 2) when scrape amount is lower than initial stake
            await token.createStake(web3.utils.toWei("5000000"), 10, ZERO_ADDRESS, { from: user1 });
            const stake = await getLastEvent("StakeStart", token);

            // fast-forward 5 days
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user2 });

            let latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const stakeDataBefore = await token.checkStakeByID(user1, latestStakeID);
            // console.log(stakeDataBefore.stakesShares);
            // console.log(stakeDataBefore.rewardAmount.toString(), 'reward before');
            // console.log(stakeDataBefore.stakedAmount.toString(), 'staked before');
            assert.equal(stakeDataBefore.stakesShares > 0, true);
            assert.equal(parseInt(stakeDataBefore.stakedAmount.toString()) > parseInt(stakeDataBefore.rewardAmount.toString()), true);
            await token.scrapeInterest(stake.stakeID, 0, { from: user1 });

            const interestScraped = await getLastEvent("InterestScraped", token);
            // console.log(interestScraped.scrapeAmount, 'scraped');
            const transfer = await getLastEvent("Transfer", token);
            // console.log(transfer.value, 'transfered');
            assert.equal(interestScraped.scrapeAmount, transfer.value);

            // stake data after scrape
            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const stakeDataAfter = await token.checkStakeByID(user1, latestStakeID);
            // console.log(stakeDataAfter.rewardAmount, 'reward left');
            const scrapedAndRemaining = parseInt(stakeDataAfter.rewardAmount.toString()) + parseInt(interestScraped.scrapeAmount.toString());
            assert.equal(scrapedAndRemaining, parseInt(stakeDataBefore.rewardAmount.toString()));
            assert.equal(parseInt(stakeDataBefore.stakesShares.toString()) > parseInt(stakeDataAfter.stakesShares.toString()), true);
            assert.equal(parseInt(stakeDataAfter.stakesShares.toString()) > 0, true);
        });

        it("should be able to scrape interest (from mature stake) without penalty", async () => {

            // -------------------
            // scenario B:
            // 2) when scrape amount is lower than initial stake
            await token.createStake(web3.utils.toWei("5000000"), 10, ZERO_ADDRESS, { from: user1 });
            const stake = await getLastEvent("StakeStart", token);

            // fast-forward 11 days
            await advanceTimeAndBlock(11 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user2 });

            let latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const stakeDataBefore = await token.checkStakeByID(user1, latestStakeID);
            // console.log(stakeDataBefore.stakesShares);
            // console.log(stakeDataBefore.rewardAmount.toString(), 'reward before');
            // console.log(stakeDataBefore.stakedAmount.toString(), 'staked before');
            assert.equal(stakeDataBefore.stakesShares > 0, true);
            assert.equal(parseInt(stakeDataBefore.stakedAmount.toString()) > parseInt(stakeDataBefore.rewardAmount.toString()), true);
            await token.scrapeInterest(stake.stakeID, 0, { from: user1 });

            let interestScraped = await getLastEvent("InterestScraped", token);
            // console.log(interestScraped.scrapeAmount, 'scraped');
            let transfer = await getLastEvent("Transfer", token);
            // console.log(transfer.value, 'transfered');
            assert.equal(interestScraped.scrapeAmount, transfer.value);

            // stake data after scrape
            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const stakeDataAfter = await token.checkStakeByID(user1, latestStakeID);
            // console.log(stakeDataAfter.rewardAmount, 'reward left');
            const scrapedAndRemaining = parseInt(stakeDataAfter.rewardAmount.toString()) + parseInt(interestScraped.scrapeAmount.toString());

            assert.equal(
                scrapedAndRemaining,
                parseInt(stakeDataBefore.rewardAmount.toString())
            );

            assert.equal(
                parseInt(stakeDataBefore.stakesShares.toString()),
                parseInt(stakeDataAfter.stakesShares.toString())
            );

            // fast-forward 5 days
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);

            await token.scrapeInterest(stake.stakeID, 0, { from: user1 });
            interestScraped = await getLastEvent("InterestScraped", token);
            // console.log(interestScraped.scrapeAmount, 'scraped');
            transfer = await getLastEvent("Transfer", token);
            // console.log(transfer.value, 'transfered');
            assert.equal(interestScraped.scrapeAmount, transfer.value);
            assert.equal(interestScraped.scrapeAmount, 0);
        });
    });
});
