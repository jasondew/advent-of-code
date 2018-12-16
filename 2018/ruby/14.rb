recipes = "37"
recipe_count = 2
elf0, elf1 = 0, 1
#target = "59414"
target = "306281"
index = 0

loop do
  index += 1
  print "." if index % 100_000 == 0

  if (last = recipes[-7, 7]) and last.match?(target)
    puts
    puts(recipe_count - target.size)
    break
  end

  elf0_recipe, elf1_recipe = recipes[elf0].to_i, recipes[elf1].to_i

  new_recipes = (elf0_recipe + elf1_recipe).to_s
  recipes += new_recipes
  recipe_count += new_recipes.size

  elf0 = (elf0 + elf0_recipe + 1) % recipe_count
  elf1 = (elf1 + elf1_recipe + 1) % recipe_count
end
