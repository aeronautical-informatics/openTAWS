@DO_367 @CLASS_C @MODE_3
Feature: Negative Climb Rate or Altitude Loss (Mode 3)
	The Mode 3 alert is intended to generate caution alerts when the aircraft loses altitude
	during take-off or go around. The loss of altitude can either be based on a negative altitude
	rate or it can be based on a measured loss of altitude.

  @ARMING_DISARMING @MOPS_283
  Scenario: Class C Equipment shall arm Mode 3 on take-off or go around.
    Given the aircraft is on a take-off segment
    Then Mode 3 shall be armed

  @ARMING_DISARMING @MOPS_283
  Scenario: Class C Equipment shall arm Mode 3 on take-off or go around.
    Given the aircraft is on a go-around segment
    Then Mode 3 shall be armed

  @ARMING_DISARMING @MOPS_284
  Scenario: Class C Equipment shall disarm Mode 3 when not in take-off and not in go around.
    Given the aircraft is not on a take-off segment
    Then Mode 3 shall not be armed

  @ARMING_DISARMING @MOPS_284
  Scenario: Class C Equipment shall disarm Mode 3 when not in take-off and not in go around.
    Given the aircraft is not on a go-around segment
    Then Mode 3 shall not be armed

  @ALERT_CRITERIA @MOPS_285
  Scenario: Class C Equipment shall generate a caution alert when either the Method 1 alert criteria are satisfied or the Method 2 alert criteria are satisfied ...
    Given not implemented

  Rule: Based on height above terrain/runway (Method 1)

    @ALERT_CRITERIA @CAUTION @MOPS_286
    Scenario Outline: Class C Equipment shall provide a caution alert when ...
      Given the aircraft is on a take-off segment
      Given Mode 3 is not inhibited
      When the rate of descent is at least <min_rate_descent> feet per minute
      When the height above terrain is between 60.0 and <max_height_terrain> feet
      Then Mode 3 is armed
      Then a Mode 3 caution shall be emitted

      Examples:
        | min_rate_descent | max_height_terrain |
        |            207.0 |               60.0 |
        |            533.0 |              600.0 |

    @ALERT_CRITERIA @CAUTION @MOPS_287
    Scenario: Class C Equipment shall not provide a Mode 3 caution alert when ... Mode 3 is not armed.
      Given the aircraft is not on a take-off segment
      Given Mode 3 is not inhibited
      When the rate of descent is at least 207.0 feet per minute
      When the height above terrain is 60.0 feet
      Then Mode 3 is not armed
      Then a Mode 3 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_287
    Scenario: Class C Equipment shall not provide a Mode 3 caution alert when ... Mode 3 is inhibited.
      Given the aircraft is on a take-off segment
      Given Mode 3 is inhibited
      When the rate of descent is at least 207.0 feet per minute
      When the height above terrain is 60.0 feet
      Then Mode 3 is armed
      Then a Mode 3 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_287
    Scenario Outline: Class C Equipment shall not provide a Mode 3 caution alert when ... within the Must Not Alert envelope ...
      Given the aircraft is on a take-off segment
      Given Mode 3 is not inhibited
      When the rate of descent is at most <max_rate_descent> feet per minute
      When the height above terrain is at least <min_height_terrain> feet
      Then Mode 3 is armed
      Then a Mode 3 caution should not be emitted

      Examples:
        | max_rate_descent | min_height_terrain |
        |             49.9 |               40.0 |
        |             49.9 |              175.0 |
        |            446.9 |              770.0 |

  Rule: Based on the combination of height above terrain/runway and altitude loss (Method 2)

    @ALERT_CRITERIA @CAUTION @MOPS_288
    Scenario Outline: Class C Equipment shall provide a caution alert when ...
      Given the aircraft is on a take-off segment
      Given Mode 3 is not inhibited
      When the height above terrain is between 60.0 <init_max_height_terrain> feet
      Given in the next phase
      When the height above terrain is at most <next_max_height_terrain> feet
      Then Mode 3 is armed
      Then a Mode 3 caution shall be emitted

      Examples:
        | init_max_height_terrain | next_max_height_terrain |
        |                    60.0 |                    34.0 |
        |                   600.0 |                   520.0 |

    @ALERT_CRITERIA @CAUTION @MOPS_289
    Scenario: Class C Equipment shall not provide a Mode 3 caution alert when ... Mode 3 is not armed.
      Given the aircraft is not on a take-off segment
      Given Mode 3 is not inhibited
      When the height above terrain is 60.0 feet
      Given in the next phase
      When the height above terrain is at 34.0 feet
      Then Mode 3 is not armed
      Then a Mode 3 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_289
    Scenario: Class C Equipment shall not provide a Mode 3 caution alert when ... Mode 3 is inhibited.
      Given the aircraft is on a take-off segment
      Given Mode 3 is inhibited
      When the height above terrain is 60.0 feet
      Given in the next phase
      When the height above terrain is at 34.0 feet
      Then Mode 3 is armed
      Then a Mode 3 caution shall not be emitted

    @ALERT_CRITERIA @CAUTION @MOPS_289
    Scenario: Class C Equipment shall not provide a Mode 3 caution alert when ...  within the Must Not Alert envelope ...
      Given the aircraft is on a take-off segment
      Given Mode 3 is not inhibited
      When the height above terrain is at least <init_min_height_terrain> feet
      Given in the next phase
      When the height above terrain is at least <next_min_height_terrain> feet
      Then Mode 3 is armed
      Then a Mode 3 caution shall not be emitted

      Examples:
        | init_min_height_terrain | next_min_height_terrain |
        |                    10.0 |                     7.1 |
        |                    60.0 |                    57.1 |
        |                  1600.0 |                  1472.1 |

    @AURAL_ALERT @CAUTION @MOPS_290
    Scenario: Class C Equipment shall be capable of generating or triggering an aural message of at least one of “Don’t Sink” and “Too Low Terrain”.
      Given not implemented

    @VISUAL_ALERT @CAUTION @MOPS_291
    Scenario: Class C Equipment shall be capable of providing an output to trigger a yellow or amber indication.
      Given not implemented
