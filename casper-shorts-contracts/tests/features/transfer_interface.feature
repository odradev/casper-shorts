Feature: Casper Shorts Transfer interface

    Scenario: Going long - transfering WCSPR to LongToken Contract 
        When Alice transfers 1 WCSPR to TokenLongContract
        Then Alice has 999 WCSPR
        Then MarketContract has 0.995 WCSPR
        Then FeeCollector has 0.005 WCSPR
        Then Alice has 0.995 LONG

    Scenario: Going short - transfering WCSPR to ShortToken Contract 
        When Alice transfers 1 WCSPR to TokenShortContract
        Then Alice has 999 WCSPR
        Then MarketContract has 0.995 WCSPR
        Then FeeCollector has 0.005 WCSPR
        Then Alice has 0.995 SHORT

    Scenario: Withdraw - transfering LONG to WCSPR Contract
        When Alice transfers 1000 WCSPR to TokenLongContract
        Then Alice has 0 WCSPR
        Then Alice has 995 LONG
        Then MarketContract has 995 WCSPR
        Then FeeCollector has 5 WCSPR

        When Alice transfers 100 LONG to TokenWCSPRContract
        # When Alice withdraws 100 LONG
        Then Alice has 895 LONG
        Then Alice has 99.5 WCSPR
        Then MarketContract has 895 WCSPR
        Then FeeCollector has 5.5 WCSPR

    Scenario: Withdraw - transfering SHORT to WCSPR Contract
        When Alice transfers 1000 WCSPR to TokenShortContract
        Then Alice has 0 WCSPR
        Then Alice has 995 SHORT
        Then MarketContract has 995 WCSPR
        Then FeeCollector has 5 WCSPR

        When Alice transfers 100 SHORT to TokenWCSPRContract
        Then Alice has 895 SHORT
        Then Alice has 99.5 WCSPR
        Then MarketContract has 895 WCSPR
        Then FeeCollector has 5.5 WCSPR
