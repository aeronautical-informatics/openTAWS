@DO_367 @CLASS_C @MODE_1
Feature: Excessive Rate of Descent (Mode 1)
	The Mode 1 alert is intended to generate caution alerts and time-critical warning alerts
	when the aircraft has a high rate of descent relative to its height above terrain. Mode 1 is
	active during all segments of flight.

  @ARMING_DISARMING @MOPS_268
  Scenario: Class C Equipment shall arm Mode 1 during the entire flight.
    Then Mode 1 shall be armed

  Rule: Standard Caution Envelope

    Background:
      Given steep approach is not selected

    @ALERT_CRITERIA @CAUTION @MOPS_269
    Scenario Outline: Class C Equipment shall provide a caution alert when ...
      Given Mode 1 is not inhibited
      When the rate of descent is at least <min_rate_descent> feet per minute
      When the height above terrain is between 100.0 and <max_height_terrain> feet
      Then a Mode 1 caution shall be emitted within 2.0 seconds

      Examples:
        | min_rate_descent | max_height_terrain |
        |           1560.0 |              100.0 |
        |           2200.0 |              630.0 |
        |           5700.0 |             2200.0 |

    @ALERT_CRITERIA @CAUTION @MOPS_270
    Scenario: Class C Equipment shall not provide a Mode 1 caution alert when ... Mode 1 is not armed.
      Given not implemented

    @ALERT_CRITERIA @CAUTION @MOPS_270
    Scenario: Class C Equipment shall not provide a Mode 1 caution alert when ... Mode 1 is inhibited.
      Given Mode 1 is inhibited
      When the rate of descent is 1560.0 feet per minute
      When the height above terrain is 100.0 feet
      Then a Mode 1 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_270
    Scenario Outline: Class C Equipment shall not provide a Mode 1 caution alert when ... within the Must Not Alert envelope ...
      Given Mode 1 is not inhibited
      When the rate of descent is at most <max_rate_descent> feet per minute
      When the height above terrain is at least <min_height_terrain> feet
      Then a Mode 1 caution shall not be emitted

      Examples:
        | max_rate_descent | min_height_terrain |
        |            963.9 |               10.0 |
        |           2299.9 |             1550.0 |
        |           4399.9 |             2900.0 |
        |           4999.9 |             3200.0 |
        |           7999.9 |             4600.0 |
        |          11999.9 |             6467.0 |

  Rule: Steep Approach Caution Envelope

    Background:
      Given steep approach is selected

    @ALERT_CRITERIA @CAUTION @MOPS_271
    Scenario Outline: Class C Equipment shall provide a caution alert when ...
      Given Mode 1 is not inhibited
      When the rate of descent is at least <min_rate_descent> feet per minute
      When the height above terrain is between 150.0 and <max_height_terrain> feet
      Then a Mode 1 caution shall be emitted within 2.0 seconds

      Examples:
        | min_rate_descent | max_height_terrain |
        |           1798.0 |              150.0 |
        |           1944.0 |              300.0 |
        |           3233.0 |             1078.0 |
        |           6225.0 |             2075.0 |

    @ALERT_CRITERIA @CAUTION @MOPS_272
    Scenario: Class C Equipment shall not provide a Mode 1 caution alert when ... Mode 1 is not armed.
      Given not implemented

    @ALERT_CRITERIA @CAUTION @MOPS_272
    Scenario: Class C Equipment shall not provide a Mode 1 caution alert when ... Mode 1 is inhibited.
      Given Mode 1 is inhibited
      When the rate of descent is 1798.0 feet per minute
      When the height above terrain 150.0 feet
      Then a Mode 1 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_272
    Scenario Outline: Class C Equipment shall not provide a Mode 1 caution alert when ... within the Must Not Alert envelope ...
      Given Mode 1 is not inhibited
      When the rate of descent is at most <max_rate_descent> feet per minute
      When the height above terrain is at least <min_height_terrain> feet
      Then a Mode 1 caution shall not be emitted

      Examples:
        | max_rate_descent | min_height_terrain |
        |            963.9 |               10.0 |
        |           2299.9 |             1550.0 |
        |           4399.9 |             2900.0 |
        |           4999.9 |             3200.0 |
        |           7999.9 |             4600.0 |
        |          11999.9 |             6467.0 |

  Rule: Standard Warning Envelope

    Background:
      Given steep approach is not selected

    @ALERT_CRITERIA @WARNING @MOPS_273
    Scenario Outline: Class C Equipment shall provide a warning alert when ...
      Given Mode 1 is not inhibited
      When the rate of descent is at least <min_rate_descent> feet per minute
      When the height above terrain is between 100.0 and <max_height_terrain> feet
      Then a Mode 1 warning shall be emitted within 2.0 seconds

      Examples:
        | min_rate_descent | max_height_terrain |
        |           1600.0 |              100.0 |
        |           1850.0 |              300.0 |
        |          10100.0 |             1958.0 |

    @ALERT_CRITERIA @WARNING @MOPS_274
    Scenario: Class C Equipment shall not provide a Mode 1 warning alert when ... Mode 1 is not armed.
      Given not implemented

    @ALERT_CRITERIA @WARNING @MOPS_274
    Scenario: Class C Equipment shall not provide a Mode 1 warning alert when ... Mode 1 is inhibited.
      Given Mode 1 is inhibited
      When the rate of descent is 1600.0 feet per minute
      When the height above terrain is 100.0 feet
      Then a Mode 1 warning shall not be emitted

    @ALERT_CRITERIA @WARNING @MOPS_274
    Scenario Outline: Class C Equipment shall not provide a Mode 1 warning alert when ... within the Must Not Alert envelope ...
      Given Mode 1 is not inhibited
      When the rate of descent is at most <max_rate_descent> feet per minute
      When the height above terrain is at least <min_height_terrain> feet
      Then a Mode 1 warning shall not be emitted

      Examples:
        | max_rate_descent | min_height_terrain |
        |           1216.9 |               10.0 |
        |           2299.9 |             1300.0 |
        |           4399.9 |             2500.0 |
        |           7999.9 |             3500.0 |
        |          11999.9 |             4611.0 |

  Rule: Steep Approach Warning Envelope

    Background:
      Given steep approach is selected

    @ALERT_CRITERIA @WARNING @MOPS_275
    Scenario Outline: Class C Equipment shall provide a warning alert when ...
      Given Mode 1 is not inhibited
      When the rate of descent is at least <min_rate_descent> feet per minute
      When the height above terrain is between 150.0 and <max_height_terrain> feet
      Then a Mode 1 warning shall be emitted within 2.0 seconds

      Examples:
        | min_rate_descent | max_height_terrain |
        |           1908.0 |              150.0 |
        |           2050.0 |              300.0 |
        |          10300.0 |             1958.0 |

    @ALERT_CRITERIA @WARNING @MOPS_276
    Scenario: Class C Equipment shall not provide a Mode 1 warning alert when ... Mode 1 is not armed.
      Given not implemented

    @ALERT_CRITERIA @WARNING @MOPS_276
    Scenario: Class C Equipment shall not provide a Mode 1 warning alert when ... Mode 1 is inhibited.
      Given Mode 1 is inhibited
      When the rate of descent is 1908.0 feet per minute
      When the height above terrain is 150.0 feet
      Then a Mode 1 warning shall not be emitted

    @ALERT_CRITERIA @WARNING @MOPS_276
    Scenario Outline: Class C Equipment shall not provide a Mode 1 warning alert when ... within the Must Not Alert envelope ...
      Given Mode 1 is not inhibited
      When the rate of descent is at most <max_rate_descent> feet per minute
      When the height above terrain is at least <min_height_terrain> feet
      Then a Mode 1 warning shall not be emitted

      Examples:
        | max_rate_descent | min_height_terrain |
        |           1216.9 |               10.0 |
        |           2299.9 |             1300.0 |
        |           4399.9 |             2500.0 |
        |           7999.9 |             3500.0 |
        |          11999.9 |             4611.0 |

  Rule: Mode 1 is armed and not-inhibited and an alert is active

    @AURAL_ALERT @CAUTION @MOPS_277
    Scenario: Class C Equipment shall be capable of generating or triggering an aural message of “Sink Rate”.
      Given not implemented

    @AURAL_ALERT @WARNING @MOPS_278
    Scenario: Class C Equipment shall be capable of generating or triggering an aural message of “Pull up”.
      Given not implemented

    @AURAL_ALERT @WARNING @MOPS_279
    Scenario: Class C Equipment shall repeat the aural message periodically for the duration of the Mode 1 warning alert condition, or until silenced by the pilot or a higher priority alert.
      Given not implemented

    @AURAL_ALERT @WARNING @MOPS_280
    Scenario: Class C Equipment shall generate a tone sweep ...
      Given not implemented

    @VISUAL_ALERT @CAUTION @MOPS_281
    Scenario: Class C Equipment shall be capable of providing an output to trigger a yellow or amber indication.
      Given not implemented

    @VISUAL_ALERT @WARNING @MOPS_282
    Scenario: Class C Equipment shall be capable of providing an output to trigger a red indication.
      Given not implemented
