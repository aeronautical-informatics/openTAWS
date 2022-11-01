@DO_367 @CLASS_C @FFAC
Feature: Five Hundred Foot Altitude Callout
Altitude callouts are a category of aural annunciation occurring at a
descending aircraft crossover of a particular height above terrain or height
above runway. The altitude callout is intended to increase flight crew
awareness of the current ground proximity. In particular, the Five Hundred Foot
Callout is required, while other altitude callouts are optional.  Besides the
Five Hundred Foot Callout, Class C Equipment may support aural messages for
other specific altitude thresholds. Examples include “four hundred”, “fifty” or
specific approach-related callouts such as “minimums” or “decision height”.
Each of these additional categories of callouts is not required.

  @ARMING_DISARMING
  Scenario: Class C Equipment should disarm altitude callouts when in take-off ...
    Given the aircraft is on a take-off segment
    Then FFAC shall not be armed

  @ARMING_DISARMING
  Scenario: Class C Equipment should disarm altitude callouts when ... in a go around.
    Given the aircraft is on a go-around segment
    Then FFAC shall not be armed

  @ARMING_DISARMING
  Scenario: Class C Equipment should arm altitude callouts when not in take-off ...
    Given the aircraft is not on a take-off segment
    Then FFAC shall be armed

  @ARMING_DISARMING
  Scenario: Class C Equipment should arm altitude callouts when not in ... go around.
    Given the aircraft is not on a go-around segment
    Then FFAC shall be armed

  @ARMING_DISARMING
  Scenario: Altitude callouts should be rearmed during the subsequent approach.
    Given the aircraft is on an approach segment
    Then FFAC shall be armed

  @AURAL_ALERT @ANNUNCIATION @MOPS_293
  Scenario: Class C Equipment shall be capable of providing or triggering a voice callout of “five hundred” ...
    Given not implemented

  Rule: Five Hundred Foot Altitude Callout above terrain

    @CALLOUT_CRITERIA @ANNUNCIATION @MOPS_292
    Scenario: Class C Equipment shall be capable of generating ... altitude callouts when ...
      Given the aircraft is on an approach segment
      Given non-precision approach is selected
      Given FFAC is not inhibited
      Given the height above terrain is at least 500 feet
      Given in the next phase
      When the height above terrain is at most 500 feet
      Then a FFAC annunciation is emitted within 1.3 seconds

  Rule: Five Hundred Foot Altitude Callout above runway

    @CALLOUT_CRITERIA @ANNUNCIATION @MOPS_292
    Scenario: Class C Equipment shall be capable of generating ... altitude callouts when ...
      Given the aircraft is on an approach segment
      Given non-precision approach is selected
      Given FFAC is not inhibited
      Given the height above runway is at least 500 feet
      Given in the next phase
      When the height above runway is at most 500 feet
      Then a FFAC annunciation is emitted within 1.3 seconds
