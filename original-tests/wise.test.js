const WiseToken = artifacts.require("WiseToken");
const LiquidityTransformer = artifacts.require("LiquidityTransformer");
const catchRevert = require("./exceptionsHelpers.js").catchRevert;

require("./utils");

const BN = web3.utils.BN;

// TESTING PARAMETERS
const SECONDS_IN_DAY = 30;
const DAILY_BONUS = 13698630136986302
const INVESTMENT_DAYS = 50;
const PRE_LAUNCH_DAYS = 0;
const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";
const PP = 1E18;

const HALF_ETH = web3.utils.toWei("0.5");
const HALF_ETH_WITH_BONUS = web3.utils.toWei("0.55");
const ONE_ETH = web3.utils.toWei("1");
const TWO_ETH = web3.utils.toWei("2");
const FIFTY_ETH = web3.utils.toWei("50");
const FIVE_ETH = web3.utils.toWei("5");
const STATIC_SUPPLY = web3.utils.toWei("5000000");
const TWO_ETH_WITH_BONUS = web3.utils.toWei("2.2"); // +10% bonus

const getLastEvent = async (eventName, instance) => {
    const events = await instance.getPastEvents(eventName, {
        fromBlock: 0,
        toBlock: "latest",
    });
    return events.pop().returnValues;
};

contract("WiseToken", ([owner, user1, user2, random]) => {
    let token;
    let launchTime;

    before(async () => {
        token = await WiseToken.new({gas: 12000000});
        await token.createPair();
    });

    describe("Initial Variables", () => {
        it("correct token name", async () => {
            const name = await token.name();
            assert.equal(name, "Wise Token");
        });

        it("correct token symbol", async () => {
            const symbol = await token.symbol();
            assert.equal(symbol, "WISE");
        });

        it("correct token decimals", async () => {
            const decimals = await token.decimals();
            assert.equal(decimals, 18);
        });

        //determine supply after initial investment phase is over
        /*it("correct initial supply", async () => {
            const supply = await token.supply();
            assert.equal(supply, 10000 * 10 ** 18);
        });*/
    });

    describe("Timing", () => {
        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            const blockInfo = await web3.eth.getBlock("latest");
            launchTime = blockInfo.timestamp + PRE_LAUNCH_DAYS * SECONDS_IN_DAY;
            // pass the investment stage
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
        });

        it("correct currentWiseDay", async () => {
            const currentWiseDay = await token.currentWiseDay();
            const blockInfoA = await web3.eth.getBlock("latest");
            assert.equal(
                currentWiseDay,
                Math.floor((blockInfoA.timestamp - launchTime) / SECONDS_IN_DAY)
            );
        });

        it("correct exact currentWiseDay", async () => {
            const currentWiseDay0 = await token.currentWiseDay();
            assert.equal(currentWiseDay0.toString(), 50);

            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            const currentWiseDay1 = await token.currentWiseDay();
            assert.equal(currentWiseDay1, 51);

            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            const currentWiseDay2 = await token.currentWiseDay();
            assert.equal(currentWiseDay2, 52);

            await advanceTimeAndBlock(3 * SECONDS_IN_DAY);
            const currentWiseDay3 = await token.currentWiseDay();
            assert.equal(currentWiseDay3, 55);
        });
    });

    describe("Staking: Start", () => {
        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            // console.log(pair, 'pair');
            lt = await LiquidityTransformer.new(token.address, pair);

            await token.setLiquidityTransfomer(lt.address);

            let blockInfo = await web3.eth.getBlock("latest");
            launchTime = blockInfo.timestamp + PRE_LAUNCH_DAYS * SECONDS_IN_DAY;

            // await lt.reserveWise([49], random, { from: user1, value: HALF_ETH });
            // pass the investment stage
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);

            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });

            // await lt.prepareReferralBonuses(0, 1, { from: user1 });
            let _currentWiseDay = await token.currentWiseDay();
            // console.log(_currentWiseDay.toString(), '_currentWiseDay');
            await lt.forwardLiquidity({ from: random });

            const data = await getLastEvent(
                "UniSwapResult",
                lt
            );

            // console.log(data);

            await lt.$getMyTokens({ from: user1 });
            const { value } = await getLastEvent(
                "Transfer",
                token
            );

            // console.log(value);
            const balanceOfUser = await token.balanceOf(user1);
            assert.equal(balanceOfUser.toString(), value.toString());
        });

        it("should be able to start stake", async () => {
            // await token.transfer(user1, web3.utils.toWei("300"));
            await token.createStake(web3.utils.toWei("200"), 1, ZERO_ADDRESS, { from: user1 });
        });

        it("should burned staked tokens", async () => {
            // await token.transfer(user1, web3.utils.toWei("300"));
            await token.createStake(web3.utils.toWei("200"), 1, ZERO_ADDRESS, { from: user1 });

            const { from, to, value } = await getLastEvent("Transfer", token);
            assert.equal(from, user1);
            assert.equal(to, ZERO_ADDRESS); // Tokens Burned
            assert.equal(value, web3.utils.toWei("200"));
        });

        it("correct stake details", async () => {

            // testing variables

            const _stakedAmount = web3.utils.toWei("100");
            const _lockDays = 365

            // await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            const totalSupy = await token.totalSupply();
            // console.log(totalSupy.toString());

            // await token.transfer(user1, _stakedAmount);
            await token.createStake(_stakedAmount, _lockDays, ZERO_ADDRESS, { from: user1 });

            let latestStakeID = await token.latestStakeID(user1, { from: user1 });
            // const ls = await token.checkMyStakeByID(stakeId);


            let ls = await token.checkStakeByID(user1, latestStakeID);

            // console.log(ls.rewardAmount.toString());

            let blockInfo = await web3.eth.getBlock("latest");

            let currentWiseDay =
                Math.floor((blockInfo.timestamp - launchTime) / SECONDS_IN_DAY);

            let _currentWiseDay = await token.currentWiseDay();
            const nextDay = currentWiseDay + 1;

            const { sharePrice } = await token.globals();
            const finalDay = nextDay + _lockDays;

            /*console.log(
                ((_stakedAmount * PP / sharePrice) + (_stakedAmount * (_lockDays * DAILY_BONUS) / 10E9) / 1E9).toString()
            );*/

            let result = ((_stakedAmount * PP / sharePrice) + (_stakedAmount * (_lockDays * DAILY_BONUS) / 10E9) / 1E9);

            // console.log(result);
            // console.log(BigInt(result));
            // console.log(BigInt(result).toString());
            // console.log(ls.stakesShares.toString());

            assert.equal(
                ls.stakesShares.toString(),
                BigInt(result).toString()
            );
            assert.equal(ls.stakedAmount, _stakedAmount);
            assert.equal(ls.rewardAmount, 0);
            assert.equal(ls.startDay, nextDay); // starting day is next day
            // assert.equal(ls.stakeID, stakeId); // think of more tests
            // assert.equal(ls.currentDay, currentWiseDay);
            assert.equal(_currentWiseDay, currentWiseDay);
            assert.equal(ls.lockDays, _lockDays);
            assert.equal(ls.finalDay.toNumber(), finalDay);
            // assert.equal(ls.daysLeft.toNumber(), finalDay - currentWiseDay);
            assert.equal(ls.penaltyAmount.toNumber(), 0);

            // assert.equal(closeDay, 0);

            assert.equal(ls.isActive, true);
            assert.equal(ls.isMature, false);

            // fast-forward 1 day pass
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            ls = await token.checkStakeByID(user1, latestStakeID);

            blockInfo = await web3.eth.getBlock("latest");

            currentWiseDay =
                Math.floor((blockInfo.timestamp - launchTime) / SECONDS_IN_DAY);

            _currentWiseDay = await token.currentWiseDay();

            assert.equal(ls.rewardAmount, 0);;
            assert.equal(_currentWiseDay, currentWiseDay);
            assert.equal(currentWiseDay, 52);
            // assert.equal(ls.daysLeft.toNumber(), finalDay - currentWiseDay);
            assert.equal(ls.rewardAmount, 0);
            assert.equal(ls.penaltyAmount.toString(), '90000000000000000000');

            // fast-forward 1 day pass
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });


            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            ls = await token.checkStakeByID(user1, latestStakeID);
            // console.log(ls.rewardAmount.toString());
            const totalSup = await token.totalSupply();
            // console.log(totalSup.toString());
            _currentWiseDay = await token.currentWiseDay();
            assert.equal(parseInt(web3.utils.fromWei(ls.rewardAmount)), 769);
            // assert.equal(ls.daysLeft.toNumber(), finalDay - _currentWiseDay);
            assert.equal(web3.utils.fromWei(ls.penaltyAmount), '89.7');

            // fast-forward 1 day pass
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            ls = await token.checkStakeByID(user1, latestStakeID);

            assert.equal(parseInt(web3.utils.fromWei(ls.rewardAmount)), 1538);
            // assert.equal(ls.daysLeft.toNumber(), 363);
            assert.equal(web3.utils.fromWei(ls.penaltyAmount), '89.5');

            // ending stake after another 1 days (demo)
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);



            await token.endStake(latestStakeID, { from: user1 });
            let _stakeId = await token.latestStakeID(user1, { from: user1 });
            let actualStake = await token.stakes(user1, _stakeId);

            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            let endedStake = await token.checkStakeByID(user1, latestStakeID);
            assert.equal(endedStake.isActive, false);
            assert.equal(endedStake.isMature, false);
            // assert.equal(endedStake.daysLeft.toNumber(), 0);
            assert.equal(parseInt(web3.utils.fromWei(endedStake.rewardAmount)), 2308);
            assert.equal(actualStake.closeDay.toNumber(), INVESTMENT_DAYS + 5);
            assert.equal(web3.utils.fromWei(ls.penaltyAmount), '89.5');

            // now another day passes - values should be the same
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            latestStakeID = await token.latestStakeID(user1, { from: user1 });
            endedStake = await token.checkStakeByID(user1, latestStakeID);
            actualStake = await token.stakes(user1, _stakeId);

            assert.equal(endedStake.isActive, false);
            assert.equal(endedStake.isMature, false);
            // assert.equal(endedStake.daysLeft.toNumber(), 0);
            assert.equal(parseInt(web3.utils.fromWei(endedStake.rewardAmount)), 2308);
            assert.equal(actualStake.closeDay.toNumber(), INVESTMENT_DAYS + 5);
        });

        it("correct stakeStart event triggered", async () => {
            // More than 1 day passes
            // await advanceTimeAndBlock(50 * SECONDS_IN_DAY);

            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 365, ZERO_ADDRESS, { from: user1 });

            const {
                stakerAddress,
                stakedAmount,
                stakesShares,
                openedTime,
                startDay,
                lockDays,
            } = await getLastEvent("StakeStart", token);

            let blockInfo = await web3.eth.getBlock("latest");
            const nextDay =
                Math.floor((blockInfo.timestamp - launchTime) / SECONDS_IN_DAY) + 1;

            const { sharePrice } = await token.globals();
            let result = ((stakedAmount * PP / sharePrice) + (stakedAmount * (lockDays * DAILY_BONUS) / 10E9) / 1E9);

            const currentWiseDay = await token.currentWiseDay();
            // console.log(currentWiseDay);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(
                stakesShares.toString(),
                BigInt(result).toString()
            );
            assert.equal(startDay, nextDay);
            assert.equal(lockDays, 365);
        });
    });

    describe("Staking: End", () => {
        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);

            await token.setLiquidityTransfomer(lt.address);

            const blockInfo = await web3.eth.getBlock("latest");
            launchTime = blockInfo.timestamp + PRE_LAUNCH_DAYS * SECONDS_IN_DAY;

            // await lt.reserveWise([49], random, { from: user1, value: HALF_ETH });
            // pass the investment stage
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);

            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });

            // await lt.prepareReferralBonuses(0, 1, { from: user1 });

            await lt.forwardLiquidity({ from: random });

            const data = await getLastEvent(
                "UniSwapResult",
                lt
            );

            //console.log(data);

            await lt.$getMyTokens({ from: user1 });
            const { value } = await getLastEvent(
                "Transfer",
                token
            );
        });

        it("should get correct stake details after ending", async () => {
            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            const _currentWiseDayWhenLocked = await token.currentWiseDay();
            const _lockDays = 365;
            await token.createStake(web3.utils.toWei("100"), _lockDays, ZERO_ADDRESS, { from: user1 });

            stakeId = await token.latestStakeID(user1, { from: user1 });

            const startBlockInfo = await web3.eth.getBlock("latest");

            // More Time passes (2 days)
            await advanceTimeAndBlock(2 * SECONDS_IN_DAY);

            // Stake ends
            const { sharePrice } = await token.globals();
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakedAmount,
                stakesShares,
                rewardAmount,
                startDay,
                finalDay,
                closeDay,
                lockDays,
                isActive,
            } = await token.stakes(user1, stakeId);

            assert.equal(stakedAmount, web3.utils.toWei("100"));

            // console.log(stakedAmount.toString(), 'stakedAmount');
            // console.log(sharePrice.toString(), 'sharePrice');

            let result = ((stakedAmount * PP / sharePrice) + (stakedAmount * (lockDays * DAILY_BONUS) / 10E9) / 1E9);

            // console.log(result,'result');

            assert.equal(
                stakesShares.toString(),
                BigInt(result).toString()
            );

            const INFLATION_RATE = await token.INFLATION_RATE();
            // console.log(INFLATION_RATE.toString(), 'INFLATION_RATE');

            assert.equal(rewardAmount.toString(), '769369874113924852772');
            assert.equal(startDay.toString(), 52); // started on wise day 52
            assert.equal(finalDay.toNumber(), parseInt(_lockDays) + parseInt(startDay)); // stake is 365 days
            assert.equal(closeDay, daysPassed);
            assert.equal(lockDays, _lockDays);
            assert.equal(isActive, false);
            assert.equal(INFLATION_RATE.toString(), '103000');
        });

        it("should get correct stakeEnd event", async () => {
            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 1, ZERO_ADDRESS, { from: user1 });

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (5 days)
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);

            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(closeDay, daysPassed);
        });

        it("should be able to end stake after 100 days", async () => {
            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 100, ZERO_ADDRESS, { from: user1 });

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(closeDay, daysPassed);
        });

        it("should be able to end stake after 365 days", async () => {
            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 365, ZERO_ADDRESS, { from: user1 });

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (50 days)
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // More Time passes (15 days)
            await advanceTimeAndBlock(15 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(closeDay, daysPassed);
        });

        it("should be able to end stake after 3000 days", async () => {
            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 3000, ZERO_ADDRESS, { from: user1 });
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (3000 days)
            for (let i = 0; i < 100; i++) {
                await advanceTimeAndBlock(30 * SECONDS_IN_DAY);
                await token.manualDailySnapshot({ from: user1 });
            }

            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(closeDay, daysPassed);
        });

        /*it("should be able to end stake after 3650 days", async () => {
            // Stake Starts
            await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), 3650, ZERO_ADDRESS, { from: user1 });
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (3000 days)
            for (let i = 0; i < 100; i++) {
                await advanceTimeAndBlock(30 * SECONDS_IN_DAY);
                await token.manualDailySnapshot({ from: user1 });
            }

            // More Time passes (650 days)
            for (let i = 0; i < 65; i++) {
                await advanceTimeAndBlock(10 * SECONDS_IN_DAY);
                await token.manualDailySnapshot({ from: user1 });
            }

            // Stake ends
            await token.endStake(stakeId, { from: user1, gasLimit: 600000000 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, web3.utils.toWei("100"));
            assert.equal(closeDay, daysPassed);
});*/

        /*it("should be able to end stake after 4000 days", async () => {
            // Stake Starts
            await token.transfer(user1, 100);
            await token.createStake(100, 4000, ZERO_ADDRESS, { from: user1 });
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (4000 days)
            for (let i = 0; i < 40; i++) {
                await advanceTimeAndBlock(100 * SECONDS_IN_DAY);
                await token.manualDailySnapshot({ from: user1 });
            }

            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const {
                stakerAddress,
                stakedAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, 100);
            assert.equal(closeDay, daysPassed);
});*/

        it("should get correct amount of tokens after ending stake", async () => {
            const STAKE_AMOUNT = web3.utils.toWei("100");
            const REWARD_AMOUNT = '769369841395460018525'; // 3% - after 10 days
            //const REWARD_AMOUNT = '769369841395460018525'; // 3% - after 10 days

            // Stake Starts
            // await token.transfer(user1, STAKE_AMOUNT);
            await token.createStake(STAKE_AMOUNT, 1, ZERO_ADDRESS, { from: user1 });
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (10 days)
            await advanceTimeAndBlock(10 * SECONDS_IN_DAY);

            // Stake ends
            const balanceBefore = await token.balanceOf(user1);
            await token.endStake(stakeId, { from: user1 });

            const { startDay, finalDay } = await token.stakes(user1, stakeId);

            let rewards = new BN("0");
            for (let i = startDay; i < finalDay; i++) {
                const { totalShares, inflationAmount } = await token.snapshots(i);
                const scheduledToEnd = await token.scheduledToEnd(i);
                const totalPenalties = await token.totalPenalties(i);

                rewards = rewards
                    .add(
                        new BN(String(STAKE_AMOUNT * 10)).div(
                            new BN(totalShares).sub(new BN(scheduledToEnd))
                        )
                    )
                    .mul(new BN(inflationAmount).add(new BN(totalPenalties)));
            }

            const { from, to, value } = await getLastEvent("Transfer", token);

            assert.equal(from, ZERO_ADDRESS);
            assert.equal(to, user1);
            // assert.equal(value, REWARD_AMOUNT); //  Latest event sends initial amount staked

            const balanceAfter = await token.balanceOf(user1);
            assert.equal(
                parseInt(web3.utils.fromWei(balanceAfter)),
                parseInt(web3.utils.fromWei(balanceBefore)) + parseInt(web3.utils.fromWei(STAKE_AMOUNT)) + parseInt(web3.utils.fromWei(REWARD_AMOUNT)));
        });

        it("should get correct immature stake details", async () => {
            // Parameters
            const LOCK_PERIOD = 15;
            const STAKED_AMOUNT = web3.utils.toWei("100");

            const endBlockInfo = await web3.eth.getBlock("latest");
            const daysPassed = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            const initialDay = daysPassed;
            const startDay = daysPassed + 1;

            // Stake Starts
            // await token.transfer(user1, STAKED_AMOUNT);
            await token.createStake(STAKED_AMOUNT, LOCK_PERIOD, ZERO_ADDRESS, { from: user1 });

            const DAYS_PASSED = 5;
            await advanceTimeAndBlock(DAYS_PASSED * SECONDS_IN_DAY);

            await token.manualDailySnapshot({ from: user1 });


            const latestStakeID = await token.latestStakeID(user1, { from: user1 });
            const latestStake = await token.checkStakeByID(user1, latestStakeID);
            const _currentWiseDay = await token.currentWiseDay();

            assert.equal(latestStake.startDay.toNumber(), startDay);
            assert.equal(latestStake.lockDays, LOCK_PERIOD);
            assert.equal(latestStake.finalDay, LOCK_PERIOD + startDay);
            assert.equal(_currentWiseDay, initialDay + DAYS_PASSED);
            // assert.equal(latestStake.daysLeft.toNumber(), latestStake.finalDay - _currentWiseDay);
            assert.equal(latestStake.isActive, true);
            assert.equal(latestStake.isMature, false);
        });

        it("should be able to end stake with penalty", async () => {
            // Parameters
            const LOCK_PERIOD = 15;
            const STAKE_AMOUNT = web3.utils.toWei("100");

            const _endBlockInfo = await web3.eth.getBlock("latest");
            const stakeStartDay = Math.floor(
                (_endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            // const initialDay = _daysPassed;
            // const startDay = _daysPassed + 1;

            // Stake Starts
            // await token.transfer(user1, STAKE_AMOUNT);
            await token.createStake(STAKE_AMOUNT, LOCK_PERIOD, ZERO_ADDRESS, { from: user1 });

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes
            const DAYS_PASSED = 3;
            await advanceTimeAndBlock(DAYS_PASSED * SECONDS_IN_DAY);


            // Stake ends
            await token.endStake(stakeId, { from: user1 });

            await token.manualDailySnapshot({ from: user1 });


            const endBlockInfo = await web3.eth.getBlock("latest");
            const currentWiseDay = Math.floor(
                (endBlockInfo.timestamp - launchTime) / SECONDS_IN_DAY
            );

            // const { startDay, finalDay } = await token.stakes(user1, stakeId);
            // console.log(Number(startDay), Number(finalDay))


            let rewards = new BN("0");
            for (let i = stakeStartDay +1 ; i < currentWiseDay; i++) {
                const { totalShares, inflationAmount } = await token.snapshots(i);
                const scheduledToEnd = await token.scheduledToEnd(i);
                const totalPenalties = await token.totalPenalties(i);

                // console.log(totalShares.toString(), inflationAmount.toString())
                // console.log(scheduledToEnd.toString(), totalPenalties.toString())

                rewards = rewards
                    .add(
                        new BN(String(STAKE_AMOUNT * 10)).div(
                            new BN(totalShares).sub(new BN(scheduledToEnd))
                        )
                    )
                    .mul(new BN(inflationAmount).add(new BN(totalPenalties)));
            }

            const {
                stakerAddress,
                stakedAmount,
                rewardAmount,
                closeDay,
                closedTime,
            } = await getLastEvent("StakeEnd", token);

            const penalty =
                (STAKE_AMOUNT *
                    (100 + (800 * (DAYS_PASSED + LOCK_PERIOD - currentWiseDay)) / LOCK_PERIOD)) /
                1000;

            assert.equal(stakerAddress, user1);
            assert.equal(stakedAmount, STAKE_AMOUNT);
            // assert.equal(rewardAmount, rewards.toString()); // no penalty reward calculation yet
            // assert.equal(closedTime, endBlockInfo.timestamp, 10);
            // assert.approximately(parseInt(closedTime), parseInt(endBlockInfo.timestamp), 10);
            assert.equal(closeDay, currentWiseDay);

            const balance = await token.balanceOf(user1);
            // assert.equal(balance, Math.floor(STAKE_AMOUNT - penalty)); // Gets back staked amount - penalty
        });

        it("should revert if user tries to end stake again", async () => {
            const LOCK_PERIOD = 1;

            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("100"));
            await token.createStake(web3.utils.toWei("100"), LOCK_PERIOD, ZERO_ADDRESS, { from: user1 });

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });

            // More Time passes (10 days)
            await advanceTimeAndBlock(10 * SECONDS_IN_DAY);

            // Stake ends
            await token.endStake(stakeId, { from: user1 });
            await catchRevert(
                token.endStake(stakeId, { from: user1 }),
                "revert WISE: not an active stake"
            );
        });

        it("should track scheduledToEnd shares in daily snapshots", async () => {

            const STAKE_AMOUT = web3.utils.toWei("100");
            const LOCK_PERIOD = 10;
            const PRECISION_RATE = 1E18;
            const SHARE_AMOUNT_DAY = 50000000000000000000 / 365; // 5% in a year
            const SHARE_AMOUNT_LOCK = SHARE_AMOUNT_DAY * LOCK_PERIOD;

            const { sharePrice } = await token.globals();

            // let EXPECTED_SHARES = STAKE_AMOUT * PRECISION_RATE / sharePrice + SHARE_AMOUNT_LOCK;
            // console.log(new BN(EXPECTED_SHARES));
            // EXPECTED_SHARES = new BN(EXPECTED_SHARES);
            // console.log(EXPECTED_SHARES / PRECISION_RATE, 'shares');
            // console.log(EXPECTED_SHARES.toString());

            // Stake Starts locking on day 51 for 10 days stake starts on day 52 and ends on day 62
            await token.createStake(STAKE_AMOUT, LOCK_PERIOD, ZERO_ADDRESS, { from: user1 });
            let currentWiseDay = await token.currentWiseDay();
            // console.log(currentWiseDay.toNumber(), 'currentWiseDay');

            let scheduledToEndShares = await token.scheduledToEnd(62);
            // console.log(scheduledToEndShares);

            // utilize stakeID
            stakeId = await token.latestStakeID(user1, { from: user1 });
            let actualStake = await token.stakes(user1, stakeId);
            // console.log(actualStake.finalDay.toString(), 'finalDay');

            // More Time passes (10 days)
            await advanceTimeAndBlock(14 * SECONDS_IN_DAY);
            await token.manualDailySnapshot({ from: user1 });

            currentWiseDay = await token.currentWiseDay();
            // console.log(currentWiseDay.toNumber(), 'currentWiseDay');
            const scheduledToEndOnCurrentDay = await token.scheduledToEnd(61);
            // console.log(scheduledToEndOnCurrentDay);

            const snapshotA = await token.snapshots(60);
            // console.log(snapshotA.totalShares.toString());
            assert.equal(snapshotA.totalShares.toString(), scheduledToEndShares.toString());
            // console.log(snapshotA, 'day60');

            const snapshotB = await token.snapshots(61);
            // console.log(snapshotB.totalShares.toString());
            assert.equal(snapshotB.totalShares.toString(), scheduledToEndShares.toString());
            assert.equal(snapshotB.scheduledToEnd.toString(), '0');
            // console.log(snapshotB, 'day61');

            // stake ends on day 62 -> despite stake is
            // not closed the scheduledToEnd balance is
            // keeping totalShares as 0 -> see snapshotC

            const snapshotC = await token.snapshots(62);
            // console.log(snapshotC, 'day62');
            assert.equal(snapshotC.totalShares.toString(), '0');
            assert.equal(snapshotC.scheduledToEnd.toString(), scheduledToEndShares.toString());

            const snapshotD = await token.snapshots(63);
            assert.equal(snapshotD.totalShares.toString(), '0');
            assert.equal(snapshotD.scheduledToEnd.toString(), scheduledToEndShares.toString());
            // console.log(snapshotD, 'day63');

            const snapshotE = await token.snapshots(64);
            // console.log(snapshotE, 'day64');
            assert.equal(snapshotE.totalShares.toString(), '0');
            assert.equal(snapshotE.scheduledToEnd.toString(), scheduledToEndShares.toString());

            // future snapshot
            const snapshotF = await token.snapshots(65);
            // console.log(snapshotF, 'day65again');
            assert.equal(snapshotF.totalShares.toString(), 0);
            assert.equal(snapshotF.inflationAmount.toString(), 0);
            assert.equal(snapshotF.scheduledToEnd.toString(), 0);

            // Stake ends
            // console.log('-----------');
            // console.log('stake-ended');
            // console.log('-----------');

            await token.endStake(stakeId, { from: user1 });

            const snapshotDAgain = await token.snapshots(63);
            // console.log(snapshotDAgain, 'day63again');
            assert.equal(snapshotDAgain.totalShares.toString(), '0');
            assert.equal(snapshotDAgain.scheduledToEnd.toString(), scheduledToEndShares.toString());

            const snapshotEAgain = await token.snapshots(64);
            // console.log(snapshotEAgain, 'day64again');
            assert.equal(snapshotEAgain.totalShares.toString(), 0);
            assert.equal(snapshotEAgain.scheduledToEnd.toString(), 0);

            // future snapshot
            const snapshotFAgain = await token.snapshots(65);
            // console.log(snapshotFAgain, 'day65again');
            assert.equal(snapshotFAgain.totalShares.toString(), 0);
            assert.equal(snapshotFAgain.inflationAmount.toString(), 0);
            assert.equal(snapshotFAgain.scheduledToEnd.toString(), 0);
        });
    });

    describe("Daily Snapshot", () => {
        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);

            await token.setLiquidityTransfomer(lt.address);

            const blockInfo = await web3.eth.getBlock("latest");
            launchTime = blockInfo.timestamp + PRE_LAUNCH_DAYS * SECONDS_IN_DAY;

            // await lt.reserveWise([49], random, { from: user1, value: HALF_ETH });
            // pass the investment stage
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);

            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });

            // await lt.prepareReferralBonuses(0, 1, { from: user1 });

            await lt.forwardLiquidity({ from: random });

            const data = await getLastEvent(
                "UniSwapResult",
                lt
            );

            // console.log(data);

            await lt.$getMyTokens({ from: user1 });
            const { value } = await getLastEvent(
                "Transfer",
                token
            );
        });

        it("correct initial global data", async () => {
            const {
                totalStaked,
                totalShares,
                sharePrice,
                currentWiseDay,
            } = await token.globals();

            assert.equal(totalStaked, 0);
            assert.equal(totalShares, 0);
            assert.equal(sharePrice, 100E15);
            assert.equal(currentWiseDay, 0);
        });

        it("stake should trigger a snapshot save", async () => {

            // 1 day passes
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);

            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("1000"));
            await token.createStake(web3.utils.toWei("100"), 365, ZERO_ADDRESS, { from: user1 });

            // More Time passes (5 days)
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);

            // Second stake triggers daily snapshot of
            // snaphost is called before this stake, so this one is not stored
            await token.createStake(web3.utils.toWei("200"), 1, ZERO_ADDRESS, { from: user1 });

            const { sharePrice, totalStaked } = await token.globals();
            // console.log(sharePrice, 'sharePrice');

            const { totalShares } = await token.snapshots(52);
            assert.equal(totalStaked.toString(), web3.utils.toWei("300"));
            assert.equal(totalShares.toString(), "1050000000000000000000");
        });
    });

    describe("Generic", () => {
        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);

            await token.setLiquidityTransfomer(lt.address);

            const blockInfo = await web3.eth.getBlock("latest");
            launchTime = blockInfo.timestamp + PRE_LAUNCH_DAYS * SECONDS_IN_DAY;

            // await lt.reserveWise([49], random, { from: user1, value: HALF_ETH });
            // pass the investment stage
            await advanceTimeAndBlock(50 * SECONDS_IN_DAY);

            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // await lt.generateSupply(49, { from: user1 });
            await lt.generateSupply(50, { from: user1 });

            // await lt.prepareReferralBonuses(0, 1, { from: user1 });

            await lt.forwardLiquidity({ from: random });

            const data = await getLastEvent(
                "UniSwapResult",
                lt
            );

            // console.log(data);

            await lt.$getMyTokens({ from: user1 });
            const { value } = await getLastEvent(
                "Transfer",
                token
            );

            // pass the investment stage
            // await advanceTimeAndBlock(50 * SECONDS_IN_DAY);
        });

        it("stake should trigger a snapshot save", async () => {
            // 1 day passes
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);

            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("1000"));
            await token.createStake(web3.utils.toWei("100"), 1825, ZERO_ADDRESS, { from: user1 });

            // More Time passes (5 days)
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);

            // Second stake triggers daily snapshot of
            // snaphost is called before this stake, so this one is not stored
            await token.createStake(web3.utils.toWei("200"), 1, ZERO_ADDRESS, { from: user1 });

            const { sharePrice, totalStaked } = await token.globals();
            // console.log(sharePrice, 'sharePrice');

            const snapshot = await token.snapshots(52);
            const globals = await token.globals();

            // console.log(snapshot.totalShares.toString());
            // console.log(snapshot.inflationAmount.toString());
            // console.log(snapshot.scheduledToEnd.toString());

            assert.equal(globals.totalStaked.toString(), web3.utils.toWei("300"));
            assert.equal(snapshot.totalShares.toString(), "1250000000000000000000");
        });

        it("stake should give correct amount of extra shares for referral address", async () => {
            // 1 day passes
            await advanceTimeAndBlock(1 * SECONDS_IN_DAY);

            // Stake Starts
            // await token.transfer(user1, web3.utils.toWei("1000"));
            await token.createStake(web3.utils.toWei("100"), 1825, random, { from: user1 });

            // More Time passes (5 days)
            await advanceTimeAndBlock(5 * SECONDS_IN_DAY);

            // Second stake triggers daily snapshot of
            // snaphost is called before this stake, so this one is not stored
            await token.createStake(web3.utils.toWei("200"), 1, ZERO_ADDRESS, { from: user1 });

            const { totalStaked, sharePrice } = await token.globals();

            const { totalShares } = await token.snapshots(52);
            assert.equal(totalStaked.toString(), web3.utils.toWei("300"));
            assert.equal(totalShares.toString(), "1350000000000000000000");
        });
    });
});
