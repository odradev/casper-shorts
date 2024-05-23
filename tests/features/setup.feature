Feature: Casper Shorts Test Setup

    Scenario Outline: Initial of <account> balances matches.
        Then <account> has 1000 WCSPR
        Then <account> has 0 LONG
        Then <account> has 0 SHORT

    Examples:
        | account |
        | Alice   |
        | Bob     |

    Scenario: Initial price
        Then price is 0.01 USD