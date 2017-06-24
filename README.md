# Teko #
Welcome to the Teko programming language.

This implementation is written in Rust.

Install teko via cargo: `cargo install teko-rs`

Here are some pieces of code to play around with:

	help

Will print a useful tip: You can see all variables in scope with

	(@variables)

Strings are made using, standard `space-and-parens-are-special` applies

	(" hello world)

Standard math operators are available

	(+ 1 2 3)
	(+ (* 8 9) 10)
	(+ (/ 92138 9) 10)
	(- (/ 12333 9) 10)

Quote arbitrary elements

	(quote a b c)

Is the input value a symbol?

	(symbol? (head (quote a)))

Pair is the new Cons:

	(pair 4 (pair 1 ()))

(PS: try doing `(pair (pair 1 ()))` and see what happens)

Head is `car` or `first` in other lisps

	(head (quote a b c))

Tail is `cdr` or `rest` in other lisps

	(tail (quote a b c))

Check whether two symbols are the same

	(same? (head (quote a)) (head (quote b)))

Define and call a function

	(define x (fn (a b c) (@msleep 500) (* a b c)))
	(x 1 2 3)

Define and call a macro, they always take 1 argument, which is the entire syntax tree inside the call

	(define my-macro (mo n (head n)))
	(my-macro (write (+ 1 2 3)) (@msleep 300000) (write (* 8 9 10)))

# Error Handling #
Oh no, that's so tough! ~ Well not in Teko:

	(define reason
		(wind
			(write (+ 1 2 () 8))
			(write (" If we get here then nothing bad happened, let's exit or something))))
	(write (" OH NO! We unwound! Why?))
	(write reason)
	(error? reason)

I think this is one of the coolest examples :P

You can also unwind manually using `unwind`:

	(wind
		(@msleep 400)
		(unwind 800)
		(write (" this never runs)))

Note that the `wind` expression above returns `800`. You can also return errors:

	(wind
		(unwind (error (" Something bad happened, so we go back to the nearest wind)))
		(write (" and we don't run the rest of the block)))

Stack traces are useful, maybe you wanna make them yourself:

	(wind
		(unwind (error (@trace)))
		(write (" and we don't run the rest of the block)))

That's all for now!
