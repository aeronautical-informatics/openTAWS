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

  @MOPS_292
  Scenario: Five hundred foot above terrain
    Given FFAC is armed
    And FFAC is not inhibited
    And non-precision approach is selected
    And the height above terrain is at least 500 foot
    Given in the next phase
    When the height above terrain is at most 500 foot
    Then a FFAC annunciation alert is emitted within 1.3 seconds

  @MOPS_292
  Scenario: Five hundred foot above nearest runway elevation
    Given FFAC is armed
    And FFAC is not inhibited
    And non-precision approach is selected
    And the nearest runway elevation is at least 500 foot
    Given in the next phase
    When the nearest runway elevation is at most 500 foot
    Then a FFAC annunciation alert is emitted within 1.3 seconds
