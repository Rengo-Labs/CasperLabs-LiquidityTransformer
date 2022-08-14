const WiseToken = artifacts.require("WiseToken");
const LiquidityTransformer = artifacts.require("LiquidityTransformer");
const catchRevert = require("./exceptionsHelpers.js").catchRevert;

require("./utils");
const BN = web3.utils.BN;

// TESTING PARAMETERS
const SECONDS_IN_DAY = 30;
const HALF_ETH = web3.utils.toWei("0.5");
const HALF_ETH_WITH_BONUS = web3.utils.toWei("0.55");
const ONE_ETH = web3.utils.toWei("1");
const TWO_ETH = web3.utils.toWei("2");
const FOUR_ETH = web3.utils.toWei("4");
const FIFTY_ETH = web3.utils.toWei("50");
const FIVE_THOUSAND_ETH = web3.utils.toWei("5000");
const FIVE_ETH = web3.utils.toWei("5");
const STATIC_SUPPLY = web3.utils.toWei("5000000");
const TWO_ETH_WITH_BONUS = web3.utils.toWei("2.2"); // +10% bonus
const FOUR_ETH_WITH_BONUS = web3.utils.toWei("4.4"); // +10% bonus
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

    before(async () => {
        token = await WiseToken.new({gas: 12000000});
        pair = await token.UNISWAP_PAIR();
        lt = await LiquidityTransformer.new(token.address, pair);
    });

    describe("Initial Token Variables", () => {
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
    });

    describe("Initial LT Variables", () => {

        it("correct token address", async () => {
            const wc = await lt.WISE_CONTRACT();
            assert.equal(wc, token.address);
        });

        it("correct WISE day", async () => {
            const ltWiseDay = await lt._currentWiseDay();
            const tokenWiseDay = await token.currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), tokenWiseDay.toNumber());
        });

        it("correct UNISWAP pair", async () => {
            const ltUNI_PAIR = await lt.UNISWAP_PAIR();
            // console.log(ltUNI_PAIR);
            const tokenUNI_PAIR = await token.UNISWAP_PAIR();
            // console.log(tokenUNI_PAIR);
            assert.equal(ltUNI_PAIR, tokenUNI_PAIR);
        });

        it("should allow to assign LT to token", async () => {
            const tokenGateKeeperBefore = await token.transformerGateKeeper();
            const ltBefore = await token.LIQUIDITY_TRANSFORMER();

            await token.setLiquidityTransfomer(lt.address);

            const tokenGateKeeperAfter = await token.transformerGateKeeper();
            const ltAfter = await token.LIQUIDITY_TRANSFORMER();

            assert.equal(ltBefore, ZERO_ADDRESS);
            assert.equal(tokenGateKeeperBefore, owner);

            assert.equal(ltAfter, lt.address);
            assert.equal(tokenGateKeeperAfter, ZERO_ADDRESS);
        });

    });

    describe("Ability to reserve tokens", () => {

        it("should call reserveWise method with correct investmentDay range", async () => {
            await catchRevert(
                lt.reserveWise([0], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: not in initial investment days range"
            );
            await catchRevert(
                lt.reserveWise([51], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: not in initial investment days range"
            );

            await lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: ONE_ETH });
            await lt.reserveWise([2], ZERO_ADDRESS, { from: user1, value: ONE_ETH });
            await lt.reserveWise([50], ZERO_ADDRESS, { from: random, value: ONE_ETH });

            investorTotalBalance = await lt.investorTotalBalance(user1);
            investorBalanceOnDayOne = await lt.investorBalances(user1, 1);
            investorBalanceOnDayTwo = await lt.investorBalances(user1, 1);

            myInvestmentAmountDayOne = await lt.myInvestmentAmount(1, { from: user1 });
            myInvestmentAmountDayTwo = await lt.myInvestmentAmount(2, { from: user1 });
            myTotalInvestmentAmount = await lt.myTotalInvestmentAmount({ from: user1 });
            dailyTotalInvestmentDayOne = await lt.dailyTotalInvestment(1);
            dailyTotalInvestmentDayTwo = await lt.dailyTotalInvestment(2);
            dailyTotalInvestmentDayFifty = await lt.dailyTotalInvestment(50);

            // uniqueInvestorOne = await lt.uniqueInvestors(0, { from: user1 });
            // uniqueInvestorTwo = await lt.uniqueInvestors(1, { from: user1 });

            fundedDays = await lt.fundedDays({ from: user1 });

            assert.equal(fundedDays.toString(), "3");

            // assert.equal(uniqueInvestorOne, user1);
            // assert.equal(uniqueInvestorTwo, random);

            assert.equal(investorBalanceOnDayOne.toString(), ONE_ETH.toString());
            assert.equal(investorBalanceOnDayTwo.toString(), ONE_ETH.toString());
            assert.equal(investorTotalBalance.toString(), TWO_ETH.toString());

            assert.equal(investorBalanceOnDayOne.toString(), myInvestmentAmountDayOne.toString());
            assert.equal(investorBalanceOnDayTwo.toString(), myInvestmentAmountDayTwo.toString());
            assert.equal(investorTotalBalance.toString(), myTotalInvestmentAmount.toString());

            assert.equal(dailyTotalInvestmentDayOne.toString(), ONE_ETH.toString());
            assert.equal(dailyTotalInvestmentDayTwo.toString(), ONE_ETH.toString());

            assert.equal(dailyTotalInvestmentDayFifty.toString(), ONE_ETH);
            assert.equal(dailyTotalInvestmentDayOne.toString(), dailyTotalInvestmentDayFifty.toString());
        });

        it("should allow to add more ETH through reserveWise method", async () => {

            await lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: ONE_ETH });
            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: ONE_ETH });

            res = await lt.dailyTotalInvestment(1);
            res2 = await lt.dailyTotalInvestment(50);

            assert.equal(res.toString(), TWO_ETH.toString());
            assert.equal(res2.toString(), TWO_ETH.toString());
            assert.equal(res.toString(), res2.toString());
        });

        it("should not allow reservations below 0.05 ETH (min)", async () => {
            await catchRevert(
                lt.reserveWise([49], ZERO_ADDRESS, { from: user1, value: web3.utils.toWei("0.04") }),
                "revert WISE: investment below minimum"
            );

            await catchRevert(
                lt.reserveWise([49], ZERO_ADDRESS, { from: user1, value: web3.utils.toWei("0.0499") }),
                "revert WISE: investment below minimum"
            );

            await lt.reserveWise([49], ZERO_ADDRESS, { from: user1, value: web3.utils.toWei("0.05") });

            res = await lt.dailyTotalInvestment(49);
            assert.equal(res.toString(), web3.utils.toWei("0.05").toString());
        });

        it("should not allow to reserve tokens on past days", async () => {

            const ltWiseDay = await lt._currentWiseDay();
            const tokenWiseDay = await token.currentWiseDay();

            // check current day (start)
            assert.equal(tokenWiseDay.toNumber(), 0);
            assert.equal(tokenWiseDay.toNumber(), ltWiseDay.toNumber());

            await lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: ONE_ETH });
            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: ONE_ETH });

            // fast forward 50+5 days
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 5) * SECONDS_IN_DAY);

            await catchRevert(
                lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            await catchRevert(
                lt.reserveWise([2], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            await catchRevert(
                lt.reserveWise([3], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            await catchRevert(
                lt.reserveWise([4], ZERO_ADDRESS, { from: user1, value: ONE_ETH }),
                "revert WISE: investment day already passed"
            );

            await lt.reserveWise([5], ZERO_ADDRESS, { from: user1, value: ONE_ETH });

            const ltWiseDayNew = await lt._currentWiseDay();
            const tokenWiseDayNew = await token.currentWiseDay();

            assert.equal(ltWiseDayNew.toNumber(), tokenWiseDayNew.toNumber());
            assert.equal(ltWiseDayNew.toNumber(), 5);

        });

        it("should allow reservations with separate referral address", async () => {
            await catchRevert(
                lt.reserveWise([10], user1, { from: user1, value: web3.utils.toWei("0.1") }),
                "revert WISE: referral must be different address"
            );

            await lt.reserveWise([10], random, { from: user1, value: web3.utils.toWei("0.1") });
            res = await lt.investorsOnDay(10);
            assert.equal(res.toString(), "1");
        });

        it("should add investment bonus 10% if referral address is provided", async () => {
            const investmentAmount = web3.utils.toWei("1"); // 1 ETH
            const investmentAmountWithBonus = web3.utils.toWei("1.1") // +10%
            await lt.reserveWise([11], random, { from: user1, value: investmentAmount});
            dailyTotalInvestment = await lt.dailyTotalInvestment(11);
            investorBalanceOnDayEleven = await lt.investorBalances(user1, 11);
            assert.equal(dailyTotalInvestment.toString(), investmentAmountWithBonus.toString());
            assert.equal(investorBalanceOnDayEleven.toString(), investmentAmountWithBonus.toString());
        });

        it("should store referrer/referral data accordingly", async () => {
            const referralAccount = user2;
            const investmentAmount = web3.utils.toWei("1");
            await lt.reserveWise([12], referralAccount, { from: user1, value: investmentAmount});

            referralAmount = await lt.referralAmount(referralAccount);
            referralAccountStoredOne = await lt.referralAccounts(0);
            referralAccountStoredTwo = await lt.referralAccounts(1);

            assert.equal(referralAmount.toString(), investmentAmount.toString());
            assert.equal(referralAccountStoredOne, random); // from previous test
            assert.equal(referralAccountStoredTwo, referralAccount);
        });
    });

    describe("Ability to use dollar-cost average function", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
        });

        it("should allow to make equal reservation across all 50 days", async () => {

            const investor = random;
            const investment = FOUR_ETH;
            const expectedDaily = investment/50;
            const allDays = [];
            for (let i = 0; i < 50; i++) {
                allDays[i] = i + 1;
            }

            await lt.reserveWise(allDays, ZERO_ADDRESS,
                { from: investor, value: investment }
            );

            myTotalInvestmentAmount = await lt.myTotalInvestmentAmount(
                { from: investor }
            )

            myInvestmentAmountAllDays = await lt.myInvestmentAmountAllDays(
                { from: investor }
            );

            investorsOnAllDays = await lt.investorsOnAllDays();
            investmentsOnAllDays = await lt.investmentsOnAllDays();

            for (let i = 1; i <= 50; i++) {
                assert.equal(
                    myInvestmentAmountAllDays[i].toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    investorsOnAllDays[i].toString(),
                    "1"
                );

                assert.equal(
                    investmentsOnAllDays[i].toString(),
                    expectedDaily.toString()
                );
            }
            assert.equal(
                myTotalInvestmentAmount,
                FOUR_ETH.toString()
            );
        });

        it("should allow to make equal reservation across all 50 days (with referral address)", async () => {

            const investor = random;
            const referral = user1;
            const investment = FOUR_ETH;
            const investmentWithBonus = FOUR_ETH_WITH_BONUS;
            const expectedDaily = (investment/50) * 1.1 // + 10%;

            const allDays = [];
            for (let i = 0; i < 50; i++) {
                allDays[i] = i + 1;
            }

            await lt.reserveWise(allDays, referral,
                { from: investor, value: investment }
            );

            myTotalInvestmentAmount = await lt.myTotalInvestmentAmount(
                { from: investor }
            )

            myInvestmentAmountAllDays = await lt.myInvestmentAmountAllDays(
                { from: investor }
            );

            investorsOnAllDays = await lt.investorsOnAllDays();
            investmentsOnAllDays = await lt.investmentsOnAllDays();
            referralAmount = await lt.referralAmount(referral);
            referralAccountOne = await lt.referralAccounts(0);

            for (let i = 1; i <= 50; i++) {
                assert.equal(
                    myInvestmentAmountAllDays[i].toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    investorsOnAllDays[i].toString(),
                    "1"
                );

                assert.equal(
                    investmentsOnAllDays[i].toString(),
                    expectedDaily.toString()
                );
            }
            assert.equal(
                myTotalInvestmentAmount.toString(),
                investmentWithBonus.toString()
            );

            assert.equal(
                referralAmount,
                investment.toString()
            );

            assert.equal(
                referral,
                referralAccountOne
            );
        });

    });

    describe("Ability to use reserveWise with multiday accordingly", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            pair = await token.UNISWAP_PAIR();
            lt = await LiquidityTransformer.new(token.address, pair);
        });

        it("should allow to make reservations for specific array of days", async () => {

            const days = [1,2,5,40,50]
            const investor = random;
            const investment = TWO_ETH;
            const expectedDaily = investment/days.length;

            await lt.reserveWise(days, ZERO_ADDRESS,
                { from: investor, value: investment }
            );

            myTotalInvestmentAmount = await lt.myTotalInvestmentAmount(
                { from: investor }
            )

            myInvestmentAmountAllDays = await lt.myInvestmentAmountAllDays(
                { from: investor }
            );

            investorsOnAllDays = await lt.investorsOnAllDays();
            investmentsOnAllDays = await lt.investmentsOnAllDays();

            for (let i = 1; i <= days.length; i++) {

                myInvestmentAmountDayX = await lt.myInvestmentAmount(days[i-1], { from: investor });
                dailyTotalInvestmentDayX = await lt.dailyTotalInvestment(days[i-1]);

                assert.equal(
                    dailyTotalInvestmentDayX.toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    myInvestmentAmountDayX.toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    myInvestmentAmountAllDays[days[i-1]].toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    investorsOnAllDays[days[i-1]].toString(),
                    "1"
                );

                assert.equal(
                    investmentsOnAllDays[days[i-1]].toString(),
                    expectedDaily.toString()
                );
            }

        });

        it("should allow to make reservations for specific array of days (with referral address)", async () => {

            const days = [7,8,4,12,33,40,22,50,1,2];
            const investor = random;
            const investment = TWO_ETH;
            const investmentWithBonus = TWO_ETH_WITH_BONUS;
            const expectedDaily = (investmentWithBonus/days.length);
            const referral = user1;

            await lt.reserveWise(days, referral,
                { from: investor, value: investment }
            );

            myTotalInvestmentAmount = await lt.myTotalInvestmentAmount(
                { from: investor }
            )

            myInvestmentAmountAllDays = await lt.myInvestmentAmountAllDays(
                { from: investor }
            );

            investorsOnAllDays = await lt.investorsOnAllDays();
            investmentsOnAllDays = await lt.investmentsOnAllDays();
            referralAmount = await lt.referralAmount(referral);
            referralAccountOne = await lt.referralAccounts(0);

            for (let i = 1; i <= days.length; i++) {

                myInvestmentAmountDayX = await lt.myInvestmentAmount(days[i-1], { from: investor });
                dailyTotalInvestmentDayX = await lt.dailyTotalInvestment(days[i-1]);

                assert.equal(
                    dailyTotalInvestmentDayX.toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    myInvestmentAmountDayX.toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    myInvestmentAmountAllDays[days[i-1]].toString(),
                    expectedDaily.toString()
                );

                assert.equal(
                    investorsOnAllDays[days[i-1]].toString(),
                    "1"
                );

                assert.equal(
                    investmentsOnAllDays[days[i-1]].toString(),
                    expectedDaily.toString()
                );
            }

            assert.equal(
                myTotalInvestmentAmount.toString(),
                investmentWithBonus.toString()
            );

            assert.equal(
                referralAmount,
                investment.toString()
            );

            assert.equal(
                referral,
                referralAccountOne
            );

        });

        it("should not allow to make reservation for wrong/passed days", async () => {

            const investor = random;
            const investment = TWO_ETH;

            // fast forward 50+5 days
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 5) * SECONDS_IN_DAY);

            await catchRevert(
                lt.reserveWise([1,2,5,40,50], ZERO_ADDRESS,
                    { from: investor, value: investment }
                ),
                "revert WISE: investment day already passed"
            );

            await catchRevert(
                lt.reserveWise([6,7,8,40,51], ZERO_ADDRESS,
                    { from: investor, value: investment }
                ),
                "revert WISE: incorrect investment day"
            );

            await catchRevert(
                lt.reserveWise([0,6,7,8,40,50], ZERO_ADDRESS,
                    { from: investor, value: investment }
                ),
                "revert WISE: incorrect investment day"
            );
        });

    });

    describe("Ability to mint reserved tokens", () => {

        beforeEach(async () => {
            token = await WiseToken.new();
            await token.createPair();
            pair = await token.UNISWAP_PAIR();
            // console.log(pair);
            // console.log(token.address);
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);
        });

        it("should allow to process minting tokens for investor", async () => {

            await lt.reserveWise([50], user1, { from: user2, value: FIVE_THOUSAND_ETH });
            await lt.reserveWise([50], user2, { from: user1, value: FIVE_THOUSAND_ETH });
            await lt.reserveWise([43], random, { from: user1, value: FIVE_THOUSAND_ETH });
            await lt.reserveWise([41], ZERO_ADDRESS, { from: user2, value: FIVE_THOUSAND_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply([50], { from: user1 });
            await lt.generateSupply([43], { from: user1 });
            await lt.generateSupply([41], { from: user1 });

            await lt.prepareReferralBonuses(0, 10, { from: user1 });

            await catchRevert(
                lt.$getMyTokens({ from: user1 }),
                "WISE: forward liquidity first"
            );

            await lt.forwardLiquidity({ from: random });
            await lt.$getMyTokens({ from: user1 });

            const { value } = await getLastEvent(
                "Transfer",
                token
            );
            // console.log(value);
        });
    });

    describe("Ability to generate referral tokens", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            pair = await token.UNISWAP_PAIR();
            // console.log(pair);
            // console.log(token.address);
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);
        });

        it("should correctly calculate referral family tokens", async () => {

            await lt.reserveWise([41], user2, { from: user1, value: ONE_ETH });
            await lt.reserveWise([43], random, { from: user1, value: ONE_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply(41, { from: user1 });
            await lt.generateSupply(43, { from: user1 });

            referralAmountA = await lt.referralAmount(user2);
            referralAmountB = await lt.referralAmount(random);

            assert.equal(referralAmountA.toString(), ONE_ETH);
            assert.equal(referralAmountB.toString(), ONE_ETH);

            await lt.prepareReferralBonuses(0, 2, { from: user1 });

            referralAmountA_after = await lt.referralAmount(user2);
            referralAmountB_after = await lt.referralAmount(random);

            assert.equal(referralAmountA_after.toString(), "0");
            assert.equal(referralAmountB_after.toString(), "0");

            referralTokensA = await lt.referralTokens(user2);
            referralTokensB = await lt.referralTokens(random);
            // console.log(referralTokensA.toString());
            // console.log(referralTokensB.toString());

            // 10mil supply / 2 ETH * 0.05 family bonus = 250,000
            const EXPECTED_AMOUNT = web3.utils.toWei("250000");

            assert.equal(referralTokensA.toString(), EXPECTED_AMOUNT);
            assert.equal(referralTokensB.toString(), EXPECTED_AMOUNT);
        });

        it("should not give referral tokens if referral amount below 1 ETH", async () => {

            await lt.reserveWise([41], user2, { from: user1, value: ONE_ETH });
            await lt.reserveWise([43], random, { from: user1, value: HALF_ETH });
            await lt.reserveWise([50], user1, { from: user2, value: HALF_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply([41], { from: user1 });
            await lt.generateSupply([43], { from: user1 });
            await lt.generateSupply([50], { from: user1 });

            referralAmountA = await lt.referralAmount(user2);
            referralAmountB = await lt.referralAmount(random);
            referralAmountC = await lt.referralAmount(user1);

            assert.equal(referralAmountA.toString(), ONE_ETH);
            assert.equal(referralAmountB.toString(), HALF_ETH);
            assert.equal(referralAmountC.toString(), HALF_ETH);

            await lt.prepareReferralBonuses(0, 3, { from: user1 });

            referralAmountA_after = await lt.referralAmount(user2);
            referralAmountB_after = await lt.referralAmount(random);
            referralAmountC_after = await lt.referralAmount(user1);

            assert.equal(referralAmountA_after.toString(), "0");
            assert.equal(referralAmountB_after.toString(), "0");
            assert.equal(referralAmountB_after.toString(), "0");

            referralTokensA = await lt.referralTokens(user2);
            referralTokensB = await lt.referralTokens(random);
            referralTokensC = await lt.referralTokens(user1);
            // console.log(referralTokensA.toString());
            // console.log(referralTokensB.toString());

            // 15mil supply / 2 ETH * 0.05 family bonus = 375,000
            const EXPECTED_AMOUNT = web3.utils.toWei("375000");

            assert.equal(referralTokensA.toString(), EXPECTED_AMOUNT);
            assert.equal(referralTokensB.toString(), "0");
            assert.equal(referralTokensC.toString(), "0");
        });

        it("should give +10% bonus payout in WISE if referred atleast 50 ETH", async () => {

            await lt.reserveWise([50], user2, { from: user1, value: FIFTY_ETH });
            await lt.reserveWise([43], random, { from: user1, value: FIVE_ETH });
            await lt.reserveWise([41], user1, { from: user2, value: FIVE_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply([50], { from: user1 });
            await lt.generateSupply([43], { from: user1 });
            await lt.generateSupply([41], { from: user1 });

            await lt.prepareReferralBonuses(0, 3, { from: user1 });

            referralTokensA = await lt.referralTokens(user2);
            referralTokensB = await lt.referralTokens(random);
            referralTokensC = await lt.referralTokens(user1);

            // 15mil supply / 60 ETH * 0.05 family bonus = 12,500
            const EXPECTED_AMOUNT_FAMILY_BONUS = web3.utils.toWei("12500");

            // 15mil supply / 60 ETH * (50*10%) family bonus = 125,000
            const EXPECTED_AMOUNT_TEN_PERCENT = web3.utils.toWei("1250000");

            //console.log(referralTokensA.toString());
            // console.log(referralTokensB.toString());
            assert.equal(referralTokensA.toString(), EXPECTED_AMOUNT_TEN_PERCENT);
            assert.equal(referralTokensB.toString(), EXPECTED_AMOUNT_FAMILY_BONUS);
            assert.equal(referralTokensC.toString(), EXPECTED_AMOUNT_FAMILY_BONUS);
        });

        it("should give CM status if referral amount is atleast 50 ETH", async () => {

            await lt.reserveWise([50], user2, { from: user1, value: FIFTY_ETH });
            await lt.reserveWise([43], random, { from: user1, value: FIVE_ETH });
            await lt.reserveWise([41], user1, { from: user2, value: FIVE_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 51) * SECONDS_IN_DAY);

            await lt.generateSupply([50], { from: user1 });
            await lt.generateSupply([43], { from: user1 });
            await lt.generateSupply([41], { from: user1 });

            await lt.prepareReferralBonuses(0, 3, { from: user1 });

            CriticalMassUserA = await token.criticalMass(user2);
            CriticalMassUserB = await token.criticalMass(random);
            CriticalMassUserC = await token.criticalMass(user1);

            const ltWiseDay = await lt._currentWiseDay();
            // level instanly at 10,000 DAI
            assert.equal(CriticalMassUserA[0].toString(), web3.utils.toWei("10000"));

            // next wise day
            const nextWiseDay = (ltWiseDay.toNumber() + 1);
            assert.equal(CriticalMassUserA[1].toString(), nextWiseDay.toString());

            assert.equal(CriticalMassUserB[0].toString(), "0");
            assert.equal(CriticalMassUserB[1].toString(), "0");

            assert.equal(CriticalMassUserC[0].toString(), "0");
            assert.equal(CriticalMassUserC[1].toString(), "0");
        });
    });

    describe("Ability to generate daily tokens before forwarding liquidity", () => {

        beforeEach(async () => {
            token = await WiseToken.new({gas: 12000000});
            pair = await token.UNISWAP_PAIR();
            // console.log(pair);
            // console.log(token.address);
            lt = await LiquidityTransformer.new(token.address, pair);
            await token.setLiquidityTransfomer(lt.address);
        });

        it("should not allow to generate supply for current/future day", async () => {

            // check total supply on day 1
            dailyTotalSupplyOnDay1 = await lt.dailyTotalSupply(1);
            assert.equal(dailyTotalSupplyOnDay1.toString(), "0");

            // cannot generate supply for not funded days in general
            await catchRevert(
                lt.generateSupply(1, { from: user1 }),
                "revert WISE: no investments on that day"
            );

            await lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: HALF_ETH });

            // check current day
            let ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "0");

            // cannot generate supply for future days
            await catchRevert(
                lt.generateSupply(1, { from: user1, value: HALF_ETH }),
                "revert WISE: investment day must be in past"
            );

            // fastforward to day 0
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS) * SECONDS_IN_DAY);

            // check current day
            ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "0");

            // fastforward to day 1
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // check current day
            ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "1");

            // cannot generate supply for current day
            await catchRevert(
                lt.generateSupply(1, { from: user1 }),
                "revert WISE: investment day must be in past"
            );

            // fastforward to day 2
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            // cannot generate supply for out of range day
            await catchRevert(
                lt.generateSupply(0, { from: user1 }),
                "revert WISE: not in initial investment days range"
            );

            // cannot generate supply for out of range day
            await catchRevert(
                lt.generateSupply(51, { from: user1 }),
                "revert WISE: not in initial investment days range"
            );

            // check current day
            ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "2");

            // can generate supply for past day
            await lt.generateSupply(1, { from: user1 });

            // check total supply on day 1
            dailyTotalSupplyOnDay1 = await lt.dailyTotalSupply(1);
            assert.equal(dailyTotalSupplyOnDay1.toString(), STATIC_SUPPLY);
        });

        it("should not allow to re-generate supply", async () => {

            // reserve some wise
            await lt.reserveWise([1], ZERO_ADDRESS, { from: user1, value: HALF_ETH });

            // fastforward to day 2
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 10) * SECONDS_IN_DAY);

            // check current day
            const ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "10");

            // can generate supply for past day
            await lt.generateSupply(1, { from: user1 });

            // check total supply on day 1
            dailyTotalSupplyOnDay1 = await lt.dailyTotalSupply(1);
            assert.equal(dailyTotalSupplyOnDay1.toString(), STATIC_SUPPLY);

            // cannot generate supply for days that already been generated
            await catchRevert(
                lt.generateSupply(1, { from: user1 }),
                "revert WISE: supply already generated"
            );

        });

        it.skip("should not allow to generate supply for other days while awaiting callback", async () => {

            // reserve some wise
            await lt.reserveWise([12], ZERO_ADDRESS, { from: user1, value: ONE_ETH });

            // fastforward to day 2
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 13) * SECONDS_IN_DAY);

            // check current day
            const ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), "13");

            // can generate supply for past day
            await lt.generateSupply(12, { from: user1 });

            // check total supply on day 12 - means still generating awaiting callback
            dailyTotalSupplyOnDay1 = await lt.dailyTotalSupply(12);
            assert.equal(dailyTotalSupplyOnDay1.toString(), "0");

            // cannot generate supply for days that still in progress
            await catchRevert(
                lt.generateSupply(12, { from: user1 }),
                "revert WISE: supply generation in progress"
            );

            // cannot generate supply if some day still in progress
            await catchRevert(
                lt.generateSupply(10, { from: user1 }),
                "revert WISE: supply generation in progress"
            );
        });

        it("should not allow to forward liquidity before referrals are pre-calculated", async () => {

            // try to forward liquidity should fail
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: ongoing investment phase"
            );

            await lt.reserveWise([41], user2, { from: user1, value: HALF_ETH });
            await lt.reserveWise([43], random, { from: user1, value: HALF_ETH });

            await lt.reserveWise([45], user2, { from: user1, value: HALF_ETH });
            await lt.reserveWise([49], random, { from: user1, value: HALF_ETH });

            // fastforward to day 50
            await advanceTimeAndBlock((PRE_LAUNCH_DAYS + 50) * SECONDS_IN_DAY);
            const ltWiseDay = await lt._currentWiseDay();
            assert.equal(ltWiseDay.toNumber(), 50);

            // still be able to reserve on day 50
            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });

            dailyTotalInvestmentDay50 = await lt.dailyTotalInvestment(50);
            assert.equal(dailyTotalInvestmentDay50.toString(), ONE_ETH.toString());

            //not be able to reserve for day 49 and below
            await catchRevert(
                lt.reserveWise([49], user2, { from: user1, value: HALF_ETH }),
                "revert WISE: investment day already passed"
            );

            await lt.reserveWise([50], ZERO_ADDRESS, { from: user1, value: HALF_ETH });
            dailyTotalInvestmentDay50 = await lt.dailyTotalInvestment(50);
            assert.equal(dailyTotalInvestmentDay50.toString(), web3.utils.toWei("1.5").toString());

            // try to forward liquidity should fail
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: ongoing investment phase"
            );

            // fastforward + 1day = to day 51
            await advanceTimeAndBlock((1) * SECONDS_IN_DAY);

            //not be able to reserve for day 50
            await catchRevert(
                lt.reserveWise([50], user2, { from: user1, value: HALF_ETH }),
                "revert WISE: investment day already passed"
            );

            //not be able to reserve for any days distributed
            await catchRevert(
                lt.reserveWise([50], user2, { from: user1, value: HALF_ETH }),
                "revert WISE: investment day already passed"
            );

            // try to forward liquidity should still fail since need to generate supply first
            await catchRevert(
                lt.forwardLiquidity({ from: random }),
                "revert WISE: must generate supply for all days"
            );

            // check day supply before generating
            let dailyTotalSupplyOnDay41 = await lt.dailyTotalSupply(41);
            let dailyTotalSupplyOnDay43 = await lt.dailyTotalSupply(43);
            let dailyTotalSupplyOnDay45 = await lt.dailyTotalSupply(45);
            let dailyTotalSupplyOnDay49 = await lt.dailyTotalSupply(49);

            assert.equal(dailyTotalSupplyOnDay41.toString(), "0");
            assert.equal(dailyTotalSupplyOnDay43.toString(), "0");
            assert.equal(dailyTotalSupplyOnDay45.toString(), "0");
            assert.equal(dailyTotalSupplyOnDay49.toString(), "0");
            // check day supply after generating

            await lt.generateSupply(41, { from: user1 });
            await lt.generateSupply(43, { from: user1 });
            await lt.generateSupply(45, { from: user1 });
            await lt.generateSupply(49, { from: user1 });

            dailyTotalSupplyOnDay41 = await lt.dailyTotalSupply(41);
            dailyTotalSupplyOnDay43 = await lt.dailyTotalSupply(43);
            dailyTotalSupplyOnDay45 = await lt.dailyTotalSupply(45);
            dailyTotalSupplyOnDay49 = await lt.dailyTotalSupply(49);

            assert.equal(dailyTotalSupplyOnDay41.toString(), STATIC_SUPPLY);
            assert.equal(dailyTotalSupplyOnDay43.toString(), STATIC_SUPPLY);
            assert.equal(dailyTotalSupplyOnDay45.toString(), STATIC_SUPPLY);
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
            let referralAmountB = await lt.referralAmount(random);

            assert.equal(referralAmountA.toString(), ONE_ETH);
            assert.equal(referralAmountB.toString(), ONE_ETH);

            await lt.prepareReferralBonuses(0, 2, { from: user1 });

            referralAmountA = await lt.referralAmount(user2);
            referralAmountB = await lt.referralAmount(random);

            assert.equal(referralAmountA.toString(), '0');
            assert.equal(referralAmountB.toString(), "0");

            referralTokensA = await lt.referralTokens(user2);
            referralTokensB = await lt.referralTokens(random);

            assert.equal(referralTokensA.toString(), '357142850000000000000000');
            assert.equal(referralTokensB.toString(), '357142850000000000000000');
            // console.log(referralTokensA.toString());
            // console.log(referralTokensB.toString());

            // await token.setLiquidityTransfomer(lt.address);
            await catchRevert(
                token.setLiquidityTransfomer(lt.address),
                "revert WISE: transformer already defined"
            );

            await catchRevert(
                lt.$getMyTokens({ from: user1 }),
                "WISE: forward liquidity first"
            );

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

            // console.log(value);
            const balanceOfUser = await token.balanceOf(user1);
            assert.equal(balanceOfUser.toString(), value.toString());
        });
    });
});
