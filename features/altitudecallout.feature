Feature: Altitude Callout
	Scenario: Aircraft under 500 feet during cruise
		Given an aircraft is in cruise phase
		When the aircraft flies under 500 feet above terrain
		Then 'Five Hundred' voice callout is given

	Scenario: Aircraft descends under 500ft
		Given an aircraft flies more than 500 feet over the ground
		When the aircraft descent under 500 feet over the ground
		Then 'Five Hundred' voice callout is given

	Scenario: Aircraft climbs over 500ft
		Given an aircraft flies less then 500 feet over the ground
		When the aircraft climbs over 500 feet over the ground
		Then 'Five Hundred' voice callout stops
