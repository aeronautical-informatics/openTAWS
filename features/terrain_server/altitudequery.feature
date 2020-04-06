Feature: Altitude Query
	Scenario: Getting an altitude at a position
		Given the terrain server is up and running
		When user makes an altitude query with the position in geographic coordinates
		Then the altitude of the terrain at that point above the mean sea level is given in meters
