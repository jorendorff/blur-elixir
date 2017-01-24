defmodule Blur.Mixfile do
  use Mix.Project

  def project do
    [app: :blur,
     version: "0.1.0",
     elixir: "~> 1.4",
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     rustler_crates: rustler_crates(),
     compilers: [:rustler] ++ Mix.compilers,
     deps: deps()]
  end

  # Configuration for the OTP application
  #
  # Type "mix help compile.app" for more information
  def application do
    # Specify extra applications you'll use from Erlang/Elixir
    [extra_applications: [:logger]]
  end

  # Dependencies can be Hex packages:
  #
  #   {:my_dep, "~> 0.3.0"}
  #
  # Or git/path repositories:
  #
  #   {:my_dep, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
  #
  # Type "mix help deps" for more examples and options
  defp deps do
    [{:rustler, "~> 0.6"}]
  end

  defp rustler_crates do
    [
      blur: [
        path: "native/blur",
        mode: (if Mix.env == :prod, do: :release, else: :debug)
      ]
    ]
  end
end
