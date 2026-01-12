# CPU Branch Prediction Testing


With write systemcall pipelined
```
This is insane.

Remember the 1/2 random branching test I wrote?

Well, The CPU predicts the right path 95% of the time. Huh :astonished::astonished:

Performance counter stats for './target/release/branch_prediction_demo':

        15,043,097      branches:u
           851,047      branch-misses:u                  #    5.66% of all branches

       0.123731564 seconds time elapsed

       0.018112000 seconds user
       0.100586000 seconds sys

```

Without write systemcall pipelined
```
```