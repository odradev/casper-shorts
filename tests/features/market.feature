Feature: Casper Shorts Market tests

    Scenario: Going long twice
        When Alice goes long with 300 WCSPR
        Then Alice has 700 WCSPR
        Then FeeCollector has 1.5 WCSPR
        Then Market has 298.5 WCSPR
        Then Alice has 298.5 LONG
        
        When Alice goes long with 100 WCSPR
        Then Alice has 600 WCSPR
        Then FeeCollector has 2 WCSPR
        Then Market has 398 WCSPR
        Then Alice has 398 LONG

    Scenario: Going short twice
        When Alice goes short with 100 WCSPR
        Then Alice has 900 WCSPR
        Then FeeCollector has 0.5 WCSPR
        Then Market has 99.5 WCSPR
        Then Alice has 99.5 SHORT
        
        When Alice goes short with 300 WCSPR
        Then Alice has 600 WCSPR
        Then FeeCollector has 2 WCSPR
        Then Market has 398 WCSPR
        Then Alice has 398 SHORT

    Scenario: Going long and withdrawing
        When Alice goes long with 300 WCSPR
        When Alice withdraws 200 LONG
        Then Alice has 98.5 LONG
        Then FeeCollector has 2.5 WCSPR
        Then Alice has 899 WCSPR
        Then Market has 98.5 WCSPR

    Scenario: Going short and withdrawing
        When Alice goes short with 100 WCSPR        
        When Alice withdraws 50 SHORT
        Then Alice has 49.5 SHORT
        Then FeeCollector has 0.75 WCSPR
        Then Alice has 949.75 WCSPR
        Then Market has 49.5 WCSPR
