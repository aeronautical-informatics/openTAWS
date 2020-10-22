Feature: Altitude Query
  Scenario: Getting an altitude at a position
    Given the terrain server is up and running
    When the user queries the altitude for a given position
    Then the altitude of the terrain at that postion is given

# vim: set ts=2 sw=2 expandtab: retab
