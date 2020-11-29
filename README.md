# Dice::nom

Another dice generator to turn text representation of dice rolls into generators. The command line interface includes many common operators; exploding dice, target numbers, etc..

## Dice Operators

* `!` - Explode. Reroll the dice if all the original dice are maximum value (e.g. `3d4!`). An optional value can be supplied and the dice are rerolled if they are all greater than or equal to the value.
* `!!` - Explode Until. Same as explode, but keep rolling so long as all dice are maximum values
* `*` - Explode Each. Reroll any die that is the maximum value 
* `**` - Explode Each Until . Same as explode each, but keep rolling so long as the die is a maximum value. An optional value can be supplied and the die is rerolled if it is greater than or equal to the value.
* `++<n>` - Add Each. Add the given value to each die rolled.
* `--<n>` - Subtract Each. Subtract the given value from each die rolled.
* ```<n>` - Take Low. Given a dice pool, keep the lowest N values.
* `^<n>` - Take High. Given a dice pool, keep the highest N values. 
* `~<n>` - Take Middle. Given a dice pool, keep the middle N values.
* `ADV` - Advantage. Roll the dice pool twice, keeping the higher pool.
* `DIS` - Disadvantage. Roll the dice pool twice, keeping the lower pool.
* `Y` - Best Group. Keep the largest group of identical values from the pool. Keep the higher value if two groups are the same size. (e.g. `5d6Y: 3, 3, 4, 4, 1 = 8`)

## Target Operators

* `[<n>]` - Target High. Rolls greater then or equal to the given value are hits and are given a value of 1, others are given a value of 0.
* `(<n>)` - Target Low. Rolls less than or equal to the given value are hits and are given a value of 1, others are given a value of 0.
* `{<n>, <m>}` - Success. If the total rolled equals or exceeds `<n>` score 1, adding 1 for each additional `<m>` rolled. `{<n>}` is the same as `{<n>, 1}`.  Unlike the target operators, this operator is calcualted against the complete dice score.

## Comparison Operators

Two pools can be compared using the `>`, `<`, `>=`, `<=`, and `=` which return 1 for success and 0 for failure. In addition the comparison `<=>` return -1 if the left side is less than the right side, 1 if the right side is greater and 0 if they are equal. 

## Usage

```
> roll --help
roll 0.1.0
Galen P.
Generates random dice rolls

USAGE:
    roll [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --count <count>        Run the generator count number of times.
    -d, --display <display>    Display the results: full, value, or chart

ARGS:
    <INPUT>    A dice roll expression is required.
```

### Expression

Display the generator, the individual dice rolled, and the calculated value. 

```
> roll -n 2 3d4\*\*
3d4**: 4, 2*, 3, 3 = 12
3d4**: 3, 3, 1 = 7
```

### Values

Display the rolled value. One value per line.

```
> roll -n 2 -d value 3d6
12
8
```

### TODO

[] library interface
[] arithmetic operators don't appear to be working

### Chart

Generate a histogram of values. First column is value. Second column is the percentage chance to get that value or higher. 

```
> roll -n 1000000 -d chart 4d6\^3
  3. 100.0: *
  4.  99.9: **
  5.  99.6: ***
  6.  98.8: *******
  7.  97.2: ************
  8.  94.3: *******************
  9.  89.5: ***************************
 10.  82.5: ************************************
 11.  73.0: ********************************************
 12.  61.6: *************************************************
 13.  48.8: ***************************************************
 14.  35.5: ***********************************************
 15.  23.1: **************************************
 16.  13.0: ****************************
 17.   5.8: ****************
 18.   1.6: *******
```

## Development Notes

This is another take on a dice roller attempting to use a slightly more formal generator definition. This was also an opportunity to use the rust [nom](https://docs.rs/nom/6.0.1/nom/) library. 

