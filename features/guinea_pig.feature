Feature: Guinea Pig
  This feature serves as a sleek demonstration of common Gherkin usage

	@db @backend @sanity
  Scenario: Name already in use
    Given a user by the name annika exists
    When a new user sings up using the username annika
    Then the sing up shall not suceed

  @backend @security
  Scenario Outline: Failed login delay
    Given a user bert exists with the password heinz
    When user bert tries to log in with the password hans <count> times
    Then login for user bert shall be blocked for <interval> seconds

	Examples:
    | count | interval |
    | 1     | 5        |
    | 4     | 5        |
    | 5     | 30       |
    | 10    | 120      |


# vim: set ts=2 sw=2 expandtab: retab: expandtab #
