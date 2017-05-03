# author_identifier

A project that uses word frequencies to determine the author of a
given text snippet.

# How It Works

A simple histogram of word frequencies can be used to help determine
authorship of a piece of text.  This program provides a (slightly) more
sophisticated method where n-grams of text map to a set of possible
subsequent words where each word has an associated frequency.

Consider the following example text:

    This is an example text.  This is the example.

Using an n-gram size where n = 2 where we process the following
mappings:

    (This, is) -> [an, the]
    (is, an) -> [example]
    (an, example) -> [text]
    (example, text.) -> [This]
    (text., This) -> [is]
    (is, the) -> [example.]

The phase "This is" is equally likely to continue with either "an" or
"the".  This mapping provides a crude encoding of the author's style and
can be used to help identify authorship if samples of the author's
texts are available for comparison.
