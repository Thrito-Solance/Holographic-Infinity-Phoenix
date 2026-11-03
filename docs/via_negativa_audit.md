# Via Negativa Audit

## Scope and evidence

This audit checks the claims in the workspace's HIP mathematics, HAN, and
asynchronous Rust blueprint documents. It distinguishes definitions that can
be implemented from properties that do not follow from those definitions.
The supplied `[cite: 1]` and `[cite: 2]` markers have no bibliography or
retrievable source attached, so they are not treated as evidence.

## 1. What it must not be: continuous calculus is not ruled out

Nothing in the specification proves that HIP cannot use continuous calculus.
In fact, the HAN and unified specifications explicitly define smooth sections,
covariant derivatives, a continuous scale parameter, differential flow
equations, and a Wetterich flow. Those claims conflict with the instruction to
prove that continuous calculus is impossible.

A software implementation can choose discrete arithmetic for reproducibility,
but that is an engineering choice, not a theorem derived from the proposed
model. Nor does modular arithmetic by itself establish the claimed
four-dimensional quaternion division algebra. The specified modulus is
`104729`, and `104729 mod 4 = 1`, not 3. More fundamentally, the Hamilton
quaternion algebra over a finite field is split, rather than a division
algebra. In this concrete construction, the nonzero quaternion `(1, 68090,
0, 0)` has norm square `1 + 68090^2 = 0 mod 104729`. Consequently, modular
quaternion inversion is partial and must reject zero norm.

For this reason the implementation must call this structure a modular
quaternion algebra/ring, not a “Galois field” or division algebra, and must
validate inverses explicitly. If a division algebra is a hard requirement, the
algebra and modulus need to be redesigned rather than merely asserted.

## 2. What should be there, but isn't: a metric-compatible bridge

The specification gives a real symmetric graph Laplacian with real
eigenvalues, and separately gives modular non-commutative multiplication.
It defines no map between their domains, no metric on the modular states, and
no error bound showing that the map preserves distances, spectra, or
contractions. A finite field has no compatible ordering that would make
“nearest” or “smallest eigenvalue” meaningful in the usual real spectral
sense.

A defensible bridge would need an explicit encoding from real graph signals
to modular vectors, a decoding or observation rule, and measurable guarantees
for approximation error and invariants. Until that is supplied, graph
topology and modular algebra are separate components; no
metric-compatible coupling is claimed.

## 3. What is there, but shouldn't be: wall-clock timing as semantics

The sample Tokio blueprint uses sleeps to pace an actor and trigger a key
change. Wall-clock scheduling is nondeterministic and is not required for
state evolution. The core transition should instead take an explicit logical
step or scale parameter, making replay and tests deterministic. Tokio remains
appropriate for message transport and task scheduling; removing sleeps from
the state semantics does not require removing Tokio. A real-time integration
may add an external pacing adapter, kept outside the deterministic core.

## 4. What isn't there, that should be: operational memory definitions

The HAN text names attractors and persistent Betti numbers, but does not define
how application data is encoded in them, how an attractor is retrieved, or how
homology classes represent bytes, records, or updates. Betti numbers count
topological features; by themselves they do not encode payloads or imply a
memory-capacity guarantee.

An implementation may separately model state as bounded graph data and
compute persistence summaries over a defined filtration. Calling attractor
sets or persistent homology “memory” requires a concrete encoding, update,
retrieval, and capacity model that is not present here.

## 5. What is entirely defined by its absence: compute and data

“Compute and data exist solely as forbidden trajectories” is not a
mathematical definition in the supplied documents. The runnable blueprint
stores positive binary values, explicit quaternion coordinates, histories,
and messages. A useful implementation must define its state, accepted inputs,
transition function, and outputs directly. Forbidden trajectories may be
specified as invariants or rejected transitions, but absence alone does not
define executable behavior.

## 6. What is broken, but keeping the system alive: Phoenix shocks

The supplied `bit_reverse(next_prime(n))` construction is not shown to be a
field automorphism: bit reversal does not generally preserve addition or
multiplication. It is also not shown to be injective over the chosen domain or
to guarantee escape from every cycle. A deterministic finite-state
transformation cannot promise infinite novelty; it must eventually repeat a
state.

A Phoenix-style reset can be implemented as a bounded, deterministic
loop-response policy, with explicit state and tests. It is not a proof of
loop elimination, “teleportation,” or novelty. The reset must itself handle
fixed points and repeated resets, for example by imposing an execution budget
and returning an explicit terminal status.

## 7. Which noise has become a load-bearing pillar: no such dependency follows

The HAN description says the regulator filters hardware noise; it does not
derive a need for thermal jitter. The Wetterich equation is a defined
functional-flow relation, not evidence that hardware noise drives its beta
function. The proposed regulator and trace also need regularity and
invertibility assumptions before they define a finite computation.

The software core should not depend on thermal or scheduler jitter. Any
stochastic perturbation should be explicit, seeded, and independently
configurable so that deterministic behavior can be tested.

## 8. The smallest negative input: zero-norm inverse collapse

The exact algebraic hazard is not an unspecified “negative input,” but any
nonzero quaternion whose norm square is zero. For the specified modulus,
`q = (1, 68090, 0, 0)` is such a counterexample:
`68090^2 = -1 mod 104729`, so `norm_sq(q) = 0`. Fermat's exponentiation then
returns zero and cannot produce an inverse. The source also claims
`104729 = 3 mod 4`, although it is `1 mod 4`.

The inverse operation must return an explicit error whenever the norm square
is zero, and must be tested with this exact vector, as well as the zero
quaternion. This check prevents a false success result; it does not make the
algebra a division algebra.

## 9. Where constraints end and the entity begins

No source or mathematical statement can prove that software has “no internal
entity.” The implementable claim is narrower: the program is an ordinary
bounded state machine whose modular coordinates, graph size, queue capacities,
and step budget are explicitly constrained. A prime modulus bounds canonical
residues; it does not bound every allocation or prove system-wide resource
safety. Graph topology likewise constrains only the graph operations actually
implemented.

## 10. The identity threshold: when this becomes a vector database

The specifications give no formal identity threshold. Removing modular
non-commutative operations alone does not make an application a vector
database: its storage, indexing, similarity metric, and retrieval behavior
also matter. A precise comparison requires observable behavior and data
models. If the implemented query path consists only of storing vectors and
retrieving them by a conventional similarity metric, with no meaningful
non-commutative operation affecting results, it is functionally a vector
retrieval system regardless of its terminology.

## Engineering disposition

Implement only testable behavior: correct modular arithmetic with checked
inversion, explicit graph routines where their inputs and outputs are defined,
and deterministic bounded actors. Treat the fiber-bundle, Wetterich, Betti
memory, hardware-noise, cryptographic-prime, and universal fault-tolerance
language as unverified motivation unless an operational mathematical model and
acceptance tests are supplied. Do not describe the resulting prototype as
proving those claims.
