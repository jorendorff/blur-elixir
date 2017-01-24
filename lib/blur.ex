defmodule Blur do
  @moduledoc """
  Documentation for Blur.
  """

  use Rustler, otp_app: :blur

  def open(_filename),         do: :erlang.nif_error(:nif_not_loaded)
  def save(_filename, _image), do: :erlang.nif_error(:nif_not_loaded)
  def blur(_image),            do: :erlang.nif_error(:nif_not_loaded)
end
