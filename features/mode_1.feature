Feature: Mode 1: Excessive Rate of Descent
  The Mode 1 alert is intended to generate caution alerts and time-critical
  warning alerts when the aircraft has a high rate of descent relative to its
  height above terrain. Mode 1 is active during all segments of flight. In
  order to reduce nuisance alerts during steep approaches, an optional set of
  alerting curves may be employed when the aircraft is performing a steep
  approach. The determination if a steep approach is in progress can either be
  based on an input to the Equipment (such as a pilot-activated steep approach
  switch) or it can be based on internal logic (such as comparison of aircraft
  position to approach profiles in a database).

  @MOPS_268
  Scenario: Mode Arming/Disarming
    Given the plane is flying
    Then Mode 1 shall be armed

    #Rule: Standard Caution Envelope (MOPS_269, MOPS_270)

    @MOPS_269
    Scenario Outline: Must Alert
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is not selected
      When the rate of descent is at least <rate of descent> feet per minute
      And the height above terrain is between 100 and <height> feet
      Then a Mode 1 caution alert is emitted within 2 seconds

      Examples:
        | rate of descent | height |
        | 1560            | 100    |
        | 2200            | 630    |
        | 5700            | 2200   |

    @MOPS_270
    Scenario: Must Not Alert when not Armed
      Given Mode 1 is not armed
      Then a Mode 1 caution alert is not emitted at all

    @MOPS_270
    Scenario: Must Not Alert when Inhibited
      Given Mode 1 is inhibited
      Then a Mode 1 caution alert is not emitted at all

    @MOPS_270
    Scenario Outline: Must Not Alert
      Given steep approach is not selected
      When the rate of descent is at most <rate of descent> feet per minute
      But the height above terrain is not between 10 and <height> feet
      Then a Mode 1 caution alert is not emitted at all

      Examples:
        | rate of descent | height |
        | 964             | 10     |
        | 2300            | 1550   |
        | 4400            | 2900   |
        | 5000            | 3200   |
        | 8000            | 4600   |
        | 12000           | 6467   |

  #Rule: Steep Approach Caution Envelope (MOPS_271, MOPS_272)

    @MOPS_271
    Scenario Outline: Must Alert
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <rate of descent> feet per minute
      And the height above terrain is between 150 and <height> feet
      Then a Mode 1 caution alert is emitted within 2 seconds

      Examples:
        | rate of descent | height |
        | 1798            | 150    |
        | 1944            | 300    |
        | 3233            | 1078   |
        | 6225            | 2075   |

    @MOPS_272
    Scenario:  Must Not Alert when not Armed
      Given Mode 1 is not armed
      Then a Mode 1 caution alert is not emitted at all

    @MOPS_272
    Scenario: Must Not Alert when Inhibited
      Given Mode 1 is inhibited
      Then a Mode 1 caution alert is not emitted at all

    @MOPS_272
    Scenario Outline: Must Not Alert
      Given steep approach is selected
      When the rate of descent is at most <rate of descent> feet per minute
      But the height above terrain is not between 10 and <height> feet
      Then a Mode 1 caution alert is not emitted at all

      Examples:
        | rate of descent | height |
        | 964             | 10     |
        | 2300            | 1550   |
        | 4400            | 2900   |
        | 5000            | 3200   |
        | 8000            | 4600   |
        | 12000           | 6467   |

  #Rule: Warning Envelope (MOPS_273, MOPS_274)

    @MOPS_273
    Scenario Outline: Must Alert
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is not selected
      When the rate of descent is at least <rate of descent> feet per minute
      And the height above terrain is between 100 and <height> feet
      Then a Mode 1 warning alert is emitted within 2 seconds

      Examples:
        | rate of descent | height |
        | 1600            | 100    |
        | 1850            | 300    |
        | 10100           | 1958   |

    @MOPS_274
    Scenario: Must Not Alert when not Armed
      Given Mode 1 is not armed
      Then a Mode 1 warning alert is not emitted at all

    @MOPS_274
    Scenario: Must Not Alert when Inhibited
      Given Mode 1 is inhibited
      Then a Mode 1 warning alert is not emitted at all

    @MOPS_274
    Scenario Outline: Must Not Alert
      Given steep approach is not selected
      When the rate of descent is at most <rate of descent> feet per minute
      But the height above terrain is not between 10 and <height> feet
      Then a Mode 1 warning alert is not emitted at all

      Examples:
        | rate of descent | height |
        | 1217            | 10     |
        | 2300            | 1300   |
        | 4400            | 2500   |
        | 8000            | 3500   |
        | 12000           | 4611   |

  #Rule: Steep Approach Warning Envelope (MOPS_275, MOPS_276)

    @MOPS_275
    Scenario Outline: Must Alert
      Given Mode 1 is armed
      And Mode 1 is not inhibited
      And steep approach is selected
      When the rate of descent is at least <rate of descent> feet per minute
      And the height above terrain is between 150 and <height> feet
      Then a Mode 1 warning alert is emitted within 2 seconds

      Examples:
        | rate of descent | height |
        | 1908            | 150    |
        | 2050            | 300    |
        | 10300           | 1958   |

    @MOPS_276
    Scenario: Must Not Alert when not Armed
      Given Mode 1 is not armed
      Then a Mode 1 warning alert is not emitted at all

    @MOPS_276
    Scenario: Must Not Alert when Inhibited
      Given Mode 1 is inhibited
      Then a Mode 1 warning alert is not emitted at all

    @MOPS_276
    Scenario Outline: Must Not Alert
      Given steep approach is selected
      When the rate of descent is at most <rate of descent> feet per minute
      But the height above terrain is not between 10 and <height> feet
      Then a Mode 1 warning alert is not emitted at all

      Examples:
        | rate of descent | height |
        | 1217            | 10     |
        | 2300            | 1300   |
        | 4400            | 2500   |
        | 8000            | 3500   |
        | 12000           | 4611   |

  #Rule: Aural Alert (MOPS_277, MOPS_278, MOPS_279, MOPS_280)

    @MOPS_277
    Scenario: Caution Alert
      Given a caution level Mode 1 alert arises
      Then an aural message "Sink Rate" shall be emitted

    @MOPS_278
    Scenario: Warning Alert
      Given a warning level Mode 1 alert arises
      Then an aural message "Pull Up" shall be emitted

    @MOPS_279
    Scenario: Repeating Warning Alert
      Given the warning level Mode 1 alert condition persists
      And the pilot didn't silence the alert
      And no higher priority alert is triggered
      Then the aural message shall be repeated periodically
 
  # Whoop-Whoop (MOPS_280) ain't gonna be testable

  #Rule: Visual Alert (MOPS_281, MOPS_282)

    @MOPS_281
    Scenario: Caution
      Given a Mode 1 caution alert is active
      Then the TAWS shall trigger a yellow or amber indicator

    @MOPS_282
    Scenario: Warning
      Given a Mode 1 warning alert is active
      Then the TAWS shall trigger a red indicator

# vim: set ts=2 sw=2 expandtab: retab: expandtab #
