# Demo program - Blur a picture 9 times

Blur.open("nashville.jpg")
|> Stream.iterate(&Blur.blur(&1))
|> Stream.with_index()
|> Stream.drop(1)
|> Stream.take(9)
|> Stream.map(fn({image, index}) ->
  Task.async(fn() ->
    filename = "blurry-#{index}.png"
    Blur.save(image, filename)
    #System.cmd("gvfs-open", [filename])
  end)
end)
|> Stream.each(&Task.await(&1, :infinity))
|> Enum.to_list()
