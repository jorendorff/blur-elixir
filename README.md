# Blur

I threw this together as an example of how to use
[Rustler](https://github.com/hansihe/Rustler/) to extend Erlang/Elixir with Rust code.

It took two hours to make, plus a half hour for a nice
[demo program](example/smear.exs).
Once the Rust code compiled, it worked, and I didn't change it again.
Pretty great!


## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `blur` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [{:blur, "~> 0.1.0"}]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/blur](https://hexdocs.pm/blur).


## Credits

Thanks to photographer [Adam Simmons](https://www.flickr.com/photos/mr-numb/28648419472/)
for contributing [nashville.jpg](nashville.jpg) to the Creative Commons.
