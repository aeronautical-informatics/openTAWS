Feature: Alert Priorization
  Class C Equipment shall (TAWS_MOPS_300) include an alert prioritization scheme to
  handle concurrent alert conditions. Note: Class C Equipment does not require prioritization with external systems such as
  TCAS, RWS, and PWS. In many of the situations that trigger a given alert in the must alert figures above, 
  multiple alerts may be triggered concurrently. It is acceptable for the TAWS to issue only the highest
  priority alert in these situations. For example, an FLTA warning may occur in a Mode 3
  caution must alert region. In this case, Class C Equipment needs to only issue the FLTA
  warning response.

    @MOPS_300
    Scenario: Alert Priorization
      When concurrent alert conditions trigger
      Then the priorization of the alerts handles the concurrency

    @MOPS_300
    Scenario: Alert Priority
      Given an alert can occur concurrently
      Then that alert has a priority