import Std
import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

/-!
Termination (mu-decreasing) proof.

Goal: if mu strictly decreases outside fixpoint, then applying `heal`
reaches a fixpoint in <= mu(detect_issues s) steps (no infinite loop).

We keep this module ASCII-only.
-/

section
  variable {State : Type} {Issue : Type}

  /-- Iterate `heal` n times. We use Nat.iterate from core. -/
  abbrev step (heal : State -> State) (n : Nat) (s : State) : State :=
    Nat.iterate heal n s

  /--
  Main termination theorem (fuel bound = mu(detect_issues s)):

  If `mu` strictly decreases whenever we are not in fixpoint, then for any state `s`,
  there exists `n <= mu(detect_issues s)` such that after n heals we are in fixpoint.
  -/
  theorem reaches_fixpoint_within_mu
    (detect_issues : State -> List Issue)
    (mu : List Issue -> Nat)
    (heal : State -> State)
    (decreases :
      forall s : State,
        Not (is_fixpoint detect_issues mu s) ->
          mu (detect_issues (heal s)) < mu (detect_issues s))
    (s : State) :
      ∃ n : Nat, n ≤ mu (detect_issues s) ∧
        is_fixpoint detect_issues mu (step heal n s) := by

    -- Strong induction on k = mu(detect_issues s)
    let k := mu (detect_issues s)

    -- We prove a stronger statement: for all s with measure = k, we can reach fixpoint within k steps.
    have hk :
      ∀ k : Nat, ∀ s : State, mu (detect_issues s) = k ->
        ∃ n : Nat, n ≤ k ∧ is_fixpoint detect_issues mu (step heal n s) := by
      intro k
      induction k using Nat.strongRecOn with
      | ind k ih =>
        intro s hs
        -- Case split on fixpoint at s
        by_cases hfix : is_fixpoint detect_issues mu s
        · refine ⟨0, ?_, ?_⟩
          · exact Nat.zero_le k
          · simpa [step, Nat.iterate] using hfix
        · -- Not fixpoint: mu decreases after one heal
          have hlt : mu (detect_issues (heal s)) < mu (detect_issues s) :=
            decreases s hfix
          -- rewrite mu(detect_issues s) = k
          have hltk : mu (detect_issues (heal s)) < k := by simpa [hs] using hlt
          let k' := mu (detect_issues (heal s))
          have hk' : k' < k := by simpa [k'] using hltk

          -- Apply IH to state (heal s) at measure k'
          have ih' := ih k' hk' (heal s) rfl
          rcases ih' with ⟨n, hnle, hnfix⟩

          -- Lift to original state: n+1 steps from s
          refine ⟨n + 1, ?_, ?_⟩
          · -- (n+1) ≤ k
            -- n ≤ k' and k' < k  ==>  n+1 ≤ k
            have h1 : n.succ ≤ k'.succ := Nat.succ_le_succ hnle
            have h2 : k'.succ ≤ k := Nat.succ_le_of_lt hk'
            exact le_trans h1 h2
          · -- fixpoint at iterate (n+1) s
            -- Nat.iterate heal (n+1) s = Nat.iterate heal n (heal s)
            -- so we can reuse hnfix
            simpa [step, Nat.iterate_succ] using hnfix

    -- Apply hk at k = mu(detect_issues s)
    have hmain := hk k s rfl
    -- Rewrite k back
    simpa [k] using hmain

end

end ReflexiveDSHA
