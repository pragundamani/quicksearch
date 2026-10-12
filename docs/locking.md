# State and cache lock recovery

Quicksearch removes a lock automatically only on Linux and only when it can
prove that the owning process is dead. It validates the bounded lock identity
and checks the recorded process ID through `/proc`.

Every automatic stale-generation removal holds an advisory exclusive lock on
the already-open private lock directory. The identity and owner liveness are
checked again while that directory guard is held. This serializes recovery so
a stale observer cannot unlink a newer live lock generation. Failure to open or
lock the directory, or any failed recheck, fails closed.

Cache recovery uses the same directory guard while it creates or recovers its
fixed `.reap` claim and while it removes the stale cache lock. An abandoned
reaper is recovered only when its owner is proven dead under that guard.

Malformed or oversized identities, symlinks, directories, sockets, and other
special entries are never removed automatically. An unknown owner-liveness
result also fails closed. The operation waits for the normal lock timeout and
leaves the entry untouched.

Other targets do not have a portable process-liveness check in this code. They
therefore fail closed: an existing lock is never removed only because it is old.
This protects a slow active writer from losing mutual exclusion. The availability
tradeoff is that a lock left by a crashed process can block the operation until a
user or administrator verifies that no owner is active and removes the lock.

Cache mutex names are derived from the absolute, lexically normalized logical
cache path. Lock identity derivation never follows a mutable cache leaf or
mutable descendant, so replacing `.quota-control` or another cache path with a
symlink cannot change the mutex used for that logical path.
