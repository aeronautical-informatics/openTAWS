Feature: Five Hundred Foot Callout
	TAWS_MOPS_292: During non-precision approaches oTAWS shall be capable of generating the Five Hundred Foot Callout within 1.3 seconds of descending through 500 feet above terrain or 500 feet above the nearest runway elevation when the altitude callout is not inhibited.

	Scenario: Aircraft less then 500 feet above the terrain
		Given Aircraft is in non-precision approach
		And Altitude callout is not inhibited
		When Aircraft descends under 500 feet above the terrain
		Then Within 1.3 seconds Five Hundred Foot Callout is given

	Scenario: Aircraft less then 500 feet above the nearest runway
		Given Aircraft is in non-precision approach
		And Altitude callout is not inhibited
		When Aircraft descends under 500 feet above the nearest runway
		Then Within 1.3 seconds Five Hundred Foot Callout is given
