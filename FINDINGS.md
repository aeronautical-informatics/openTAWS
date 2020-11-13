# Problems

+ Aicraft Standards tend to describe the plane as one system. Hence the
  requirements range from describing inner logic of the TAWS all the way up to
  the color or visual indicators. Not all properties of the system _aircraft_
  can be checked against a software library, which by its very nature does not
  allow for any physical interaction. Examples for this can often be identified
  by the description of any interaction betwen the pilot and the TAWS.

+ + Some MOPS, for example `MOPS_270`, require that inner state of the TAWS is
  mutated. This creates a tricky problem. Implementation wise the cucumber
  framework serves as integration test. Therefore it is presented the same API
  as any other consumer of the library. The standard proclaims no use case
  where the library consumer arms/disarms _Mode 1_, hence it seems to be
  reasonable decision to not offer any capability to modify the arm state of
  _Mode 1_. If however such a capability is not present, `MOPS_270` can hardly
  be tested (as it requires _Mode 1_ to be disarmed, while _Mode 1_ is to be
  armed at any time). A workaround for this could be conditional compilation,
  where the public API of the TAWS library is extend by functions to modify the
  inner state as needed by the scenarios. This however bears a new danger: Now
  the tests are testing a different API then what is to be used in production,
  depending on ones definition even another library.

+ Only very few gherkin sentences describe a full transaction on the system.
  Because of that multiple sentences must be evaluated in order to agglomerate
  the state needed to perform what is only a single function call on the TAWS
  (e.g. pushing one `AicraftState` frame). This hinders the readability of the
  step implementations, but is not a show stopper.

+ It is not obvios how the contents of the standard is mapped to rules &
  scenarios best. 
