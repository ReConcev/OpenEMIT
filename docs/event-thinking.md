# "Event Thinking" -- The Hollywood Principle


|![Wolfman Jack 1977](https://i.imgur.com/7uIHXLo.jpg =190x) | <iframe width="150" height="99" src="https://www.youtube.com/embed/i4njPe2_rho" frameborder="0"></iframe><p><p>Wolfman Jack says:<p/><p/>"Don't call us.<br/>  &nbsp; &nbsp; &nbsp; &nbsp; We'll call you." |
| -- | -- |

>###### tags: `streaming` `actor` `EDA`
>
>[name=Author: George Willis] [time=Thu, Apr 19, 2018]

---

### What is the Hollywood Principle?
An **"Inversion of Invocation" design pattern** (for those familiar with the IoC design pattern.)


::: info
:trophy: Transform "<u>**Explicit Invocations**</u>" into "<u>**Implicit Invocations**</u>"
:::

### Explicit Invocation
"The way you do it now."

**Tightly coupled: The caller specifies the target**
* coupling to `add()` is hardcoded in all cases below
```
result = add(1,2)          // Procedural style
    
result = (int 1).add(2)    // OOP style

add(resultCallback(),1,2)  // Async style
```
### Explicit Invocation -- Deep Call Stacks
![Recursive Call Stack](https://ptrthomas.files.wordpress.com/2006/06/jtrac-callstack1.png?w=630&zoom=2 =600x)

### Explicit Invocation -- "Pass Back"
* Results must be routed back, not forward
  * Inherent in recursive **orchestration** models

    ![Results backward vs Results forward](https://www.ebayinc.com/assets/Uploads/Blog/2017/08/request-stack-trace.png)

* Context held "open"
  * deep stacks => large context
  * RMI => contextual locks by other hosts
    *  **But those hosts could fail?  What about hung context?  How is that handled?**
        *  Timeout Exceptions (Identify)
        *  Recover to previous workflow "state" and restart from waypoint (Remediate)

:::danger
### :rotating_light: &nbsp; "Wait -- we don't program around state transitions any more than we invoke asynchronously!

#### We need to rethink distributed state from the ground up!
:::

### Software Evolution
* **Embarrasing industry trends**
  * Majority of POs state:
    * (temporal) quality goes down with each release
    * (capacity) quality goes down as system grows (scaling)
  * Many IT shops adopt a "fair of failure" mentality and "sunset" systems based on refactoring not requirements

:::info
:key:Technical debt of refactoring and extending is too high
:::

### Implicit Invocation ("We'll call you")

::: info
**A foundational shift -- what Gartner calls "*Event Thinking***". </br>
**Publish/Listen** -- a cousin to **Pub/Sub**</br>
Think "Hooks" for <u>loose coupling</u> &nbsp; :fishing_pole_and_fish: 
:::

* #### Hook ![Hook](https://upload.wikimedia.org/wikipedia/commons/f/f9/Grappling_hook_%28PSF%29.png =60x30):    I'll use {--- in homage to markdown :smile:
```
myEvent.publish(aTopic):
    
  {--- aTopic.aListener1                  // Listeners can filter (optional)
  {--- aTopic.anActor2                    // Ingest/Emit Contract Queue Workers
  {--- aTopic.aListener3.anActor3         // Filtered Actor
    
=========================================================================================

dataCollectedEvent = [...] 
dataCollectedEvent.publish(Users.Create)  // Emit trigger event
   
// Users.Create:dataCollectedEvent Listeners
  {--- addCredentialsToLDAP               // Ingest into Active Directory
                                          // Emit Users.Create.Credentials:savedEvent
  {--- addEmailToMailgun                  // Ingest into User Notification domain
  {--- addToDiscus                        // Ingest into Social Engagement domain
      
// Users.Create.Credentials:savedEvent Listeners
  {--- sendConfirmationEmailToUser
      
// Users.Create Multi-Event "Merge" Listeners
  {--- EndProcess(span=2s)                // Upon completion of several parallel Actors/subflows, Emit User.Create:endFlowEvent
```

### Value Proposition

<iframe width="560" height="315" src="https://www.youtube.com/embed/v1MS2Eas1qo" frameborder="0"></iframe>

:::info
**Evolutionary Architectural Style -- <u>Achieves loose-coupling of autonomous execution bundles</u>** (container runtimes) by moving coupling from "callers" (a.k.a. Commanders, Orchestrators) to <u>autonomous listeners</u> known as Actors (Choreography, flocks of birds). Each Actor hooks into an event stream. &nbsp; :fishing_pole_and_fish: 

  * Extend via new hooks to existing events without impacting existing workflows -- just like in ETL

  * Solves coupling isues where other approaches like Service Registry (ESBs) and Service Discovery solutions:

    *  move the issue to a mediation layer (reduces impact of change, but does not remove the need to reconfigure)

    *  add additional machinery, complexity, latency and resource utilization to an otherwise lean microprocess

  *  #### Solves coupling issues at the fundamental and foundational level -- invocation.
:::

* **Naturally Async()** -- no "blocking"
  * Foundationally faster and more efficient
  * Results move forward -- not back
    * Think "incredibly fast bucket brigades"

* **Naturally Efficient** -- deep, resursive call stacks of locked, distributed context (resources) replaced with atomic, ephemeral  Actor Lifecycle context

* **Naturally Resilient** -- in contrast, **Orchestration is a <u>Resilience Antipattern</u>**, due to:
  *  critical dependency of all services/steps/transformations on one component 
  *  locked context on multiple machine of the distributed call stack when a call fails to return (syncronous)
  *  incremental state backups not available for instant recovery (remediation) -- not part of the "style"
  *  incremental monitoring not a fundamental capability, and parked on most DevOps Roadmaps (alarming)

* **Naturally Visible** -- Day 1 Monitoring of each state change, filtered by topic(s), subtopic, Domain Entities.  They are called "listeners", and they are built into the runtime as a foundational component.

* **Naturally Scalable** -- "'cause concurrency ain't easy, so stop sharing forks" -- employ **<u>Parallel Processing</u>** instead.

  * Scaling workload is all about distributing workload, and if workers keep sharing workspace resources, the result is predictively **Contention@Scale**.

  * An autonomous (atomically isolation) multi-tasking environment is required.  This is why **Container Multitenancy** is another **"Pillar of Digital Transformation"**

:::info
* **Naturally BPM Aligned:** Naturally Decomposes Business Processes/Steps into Topics/Events
:::

# Event-Driven Architecture
:::success
 :dart: **"Event-Driven Architecture is the optimal and natural solution to Recovery, Scaling and Velocity  -- Event Thinking, the foundation of true Digital Transformation."</br> ~-George~ ~Willis~**

 *[Event-Driven Architecture]: An Evolutionary Architectural Style based on Event Sourcing and CQRS design patterns.  More to come...
 *[Event Thinking]: Gartner's term for the foundational concepts around Event-Driven Architectures
:::

:::info
**Recovery Axiom:** 
## Tracking **"state transition of processing" is required** to enable recovery from prior recorded state -- like nautical waypoints.

* ### Recovery is a key <u>Resiliency/Availability</u> concept.
* ### Failures on commodity devices are a statistical certainty.
* ### Must achieve <u>HADR-as-a-Service</u>.

### :key: "<u>Embrace Failure</u>" by deploying innate recovery of prior state.
:::

:rotating_light: If the last state saved is hours old, you lose hours of processing.

#### :rocket: If saved state is millaseconds old, you recover instantly!

:::info
<i class="fa fa-edit fa-fw"></i> **Events are** fine-grained, immutable <u>**state transition ledger entries**</u> recorded in a **Distributed Log**. 
:::
*[Distributed Log]: Distributed logs like Kafka "linearly scale" on commodity hardware and in cloud VMs (Open IaaS).  Blockchain (think BitCoin) adds immutability support to the ledger through encryption, opening up B2B EDA.

### Deep Dive into Event-Driven Microservices
<iframe width="560" height="315" src="https://www.youtube.com/embed/IR1NLfaq7PU" frameborder="0"></iframe>


### Summary Comparison of Invocational Styles
| Properties | Explicit Invocation| Implicit Invocation| Notes |
| ---------- | -------- | -------- | ----- |
| Complexity | High | Low | Events are simple
| Evolution  | Low  | High | POs currently report quality issues over releases.
| Scaling</br>Multi-tasking | Concurrency | Parallelism | The key to Scalabinf workload.
| Microservices  <p/><p/> Execution Coupling (Flow) | Centralized Orchestration[^Choreo] | Independent Choreography with Oversight | Flocks are choreographed, independent actors.<p><p>What if orchestrator goes down?  More dependency.
  |BPM Alignment | Low </br> No clear model | High |
|Resilience@Scale | Low | High | Today's systems do not solve "First things First" -- **Resilience@Scale**.  <p/> <p/> Foundational problems of lean scaling, host isolation, host failure, network partitioning, snd others are not solved by the platform -- "out-of-the-box".
| Visibility</br>Monitoring | Extra.  Add Monitoring Traffic, Infrastructure, and Integration Code | Foundational</br>Listeners | 
| Process Efficiency   | Distributed Call Stacks[^stack], Synchronous "blocking" by default, Return to Caller ("Pass Back") | Actor Context, Async by design, notify/emit ("Pass Forward") |
| Process Integrity | Currently, Ingestion throttling involves discarding invocations to maintain Transactional rates | Ingestion throttling involves backpressure on upstream event publishers to preserve all invocations |
| Network Efficiency   | WET[^DRY].  Requires client caching to be DRY, and that's extra | DRY[^DRY] "Don't Repeat Yourself", Content-Centric Networking (CCN) | Events are communicated "once", while in CPU cache!
| Theorhetical Background | REST Dissertation[^REST] | Promise Theory[^PT1], Configuration Management (CM)[^PT2], Smalltalk MVC | "Events" have been around as long as Ethernet! (Xerox PARC)

[^DRY]: [Wikipedia: Don't Repeat Yourself (DRY Principle)](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself#DRY_vs_WET_solutions)

[^Choreo]: [Youtube: "Who Needs Orchestration?  What You Want is Choreography."](https://www.youtube.com/watch?v=kfF9IATUask)

[^REST]: [2000 Roy Fielding: "Architectural Styles and
the Design of Network-based Software Architectures"](https://www.ics.uci.edu/~fielding/pubs/dissertation/top.htm)

[^PT1]: [Wikipedia: Promise Theory](https://en.wikipedia.org/wiki/Promise_theory)

[^PT2]: [Mark Burgess: Promise Theory -- What is it?](https://www.linuxjournal.com/content/promise-theory%E2%80%94what-it)

[^stack]: [Lightbend/Akka: "How the Actor Model Meets the Needs of Modern, Distributed Systems"](https://doc.akka.io/docs/akka/current/guide/actors-motivation.html#the-illusion-of-a-call-stack)

