Feature: Mode 1: excessive Rate of Descent

  Scenario: Arming/Disarming (MOPS_268)
    When the plane is flying
    Then Mode 1 shall be armed

  Rule: Standard Caution Envelope (MOPS_269, MOPS_270)
    Scenario Outline: Must Alert (MOPS_269)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is no selected
      When the rate of descent is at least <sinkrate> feet per minute
      And the height above terrain is 
      * at least 100 feet
      * at most <height> feet
      Then a caution alert is emitted within 2 seconds

      Examples:
        | sinkrate | height |
        | 1560     | 100    |
        | 2200     | 630    |
        | 5700     | 100    |
  
    Scenario: Must Not Alert when not Armed (MOPS_270)
      Given Mode 1 is not armed
      Then a caution alert is not emitted at all

    Scenario: Must Not Alert when Inhibited (MOPS_270)
      Given Mode 1 is inhibited
      Then a caution alert is not emitted at all

    Scenario Outline: Must Not Alert (MOPS_270)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is no selected
      When the rate of descent is at least <sinkrate> feet per minute
      But the height above terrain is not in between 10 feet and <height> feet
      Then a caution alert is not emitted at all

      Examples:
        | sinkrate | height |
        | 964      | 10     |
        | 2300     | 1550   |
        | 4400     | 2900   |
        | 5000     | 3200   |
        | 8000     | 4600   |
        | 12000    | 6467   |

  Rule: Steep Approach Caution Envelope (MOPS_271, MOPS_272)
    Scenario Outline: Must Alert (MOPS_271)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <sinkrate> feet per minute
      And the height above terrain is 
      * at least 150 feet
      * at most <height> feet
      Then a caution alert is emitted within 2 seconds

      Examples:
        | sinkrate | height |
        | 1798     | 150    |
        | 1944     | 300    |
        | 3233     | 1078   |
        | 6225     | 2075   |

    Scenario:  Must Not Alert when not Armed (MOPS_272)
      Given Mode 1 is not armed
      Then a caution alert is not emitted at all

    Scenario: Must Not Alert when Inhibited (MOPS_272)
      Given Mode 1 is inhibited
      Then a caution alert is not emitted at all

    Scenario Outline: Must Not Alert (MOPS_272)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <sinkrate> feet per minute
      But the height above terrain is not in between 10 feet and <height> feet
      Then a caution alert is not emitted at all

      Examples:
        | sinkrate | height |
        | 964      | 10     |
        | 2300     | 1550   |
        | 4400     | 2900   |
        | 5000     | 3200   |
        | 8000     | 4600   |
        | 12000    | 6467   |

  Rule: Warning Envelope (MOPS_273, MOPS_274)
    Scenario Outline: Must Alert (MOPS_273)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is not selected
      When the rate of descent is at least <sinkrate> feet per minute
      And the height above terrain is 
      * at least 100 feet
      * at most <height> feet
      Then a caution alert is emitted within 2 seconds

      Examples:
        | sinkrate | height |
        | 1600     | 100    |
        | 1850     | 300    |
        | 10100    | 1958   |

    Scenario: Must Not Alert when not Armed (MOPS_274)
      Given Mode 1 is not armed
      Then a caution alert is not emitted at all

    Scenario: Must Not Alert when Inhibited (MOPS_274)
      Given Mode 1 is inhibited
      Then a caution alert is not emitted at all

    Scenario Outline: Must Not Alert (MOPS_274)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is not selected
      When the rate of descent is at least <sinkrate> feet per minute
      But the height above terrain is not in between 10 feet and <height> feet
      Then a caution alert is not emitted at all

      Examples:
        | sinkrate | height |
        | 1217     | 10     |
        | 2300     | 1300   |
        | 4400     | 2500   |
        | 8000     | 3500   |
        | 12000    | 4611   |

  Rule: Steep Approach Warning Envelope (MOPS_275, MOPS_276)
    Scenario Outline: Must Alert (MOPS_275)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <sinkrate> feet per minute
      And the height above terrain is 
      * at least 150 feet
      * at most <height> feet
      Then a caution alert is emitted within 2 seconds

      Examples:
        | sinkrate | height |
        | 1908     | 150    |
        | 2050     | 300    |
        | 10300    | 1958   |

    Scenario: Must Not Alert when not Armed (MOPS_276)
      Given Mode 1 is not armed
      Then a caution alert is not emitted at all

    Scenario: Must Not Alert when Inhibited (MOPS_276)
      Given Mode 1 is inhibited
      Then a caution alert is not emitted at all

    Scenario Outline: Must Not Alert (MOPS_276)
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <sinkrate> feet per minute
      But the height above terrain is not in between 10 feet and <height> feet
      Then a caution alert is not emitted at all

      Examples:
        | sinkrate | height |
        | 1217     | 10     |
        | 2300     | 1300   |
        | 4400     | 2500   |
        | 8000     | 3500   |
        | 12000    | 4611   |

  Rule: Aural Alert (MOPS_277, MOPS_278, MOPS_279, MOPS_280)
    Scenario: Caution Alert (MOPS_277)
    Given a caution level Mode 1 alert arises
    Then an aural message "Sink Rate" shall be emitted

    Scenario: Warning Alert (MOPS_278)
    Given a warning level Mode 1 alert arises
    Then an aural message "Pull Up" shall be emitted

    Scenario: Repeating Warning Alert (MOPS_279)
    Given the warning level Mode 1 alert condition persists
    Then the aural message shall be repeated periodically

    # Whoop-Whoop (MOPS_280) ain't gonna be testable
    
  Rule: Visual Alert (MOPS_281, MOPS_282)
    Scenario: Caution
    Given a caution level Mode 1 alert is active
    Then the TAWS shall trigger a yellow or amber indicator

    Scenario: Warning
    Given a warning level Mode 1 alert is active
    Then the TAWS shall trigger a red indicator

# vim: set ts=2 sw=2 expandtab: retab: expandtab #
