list=eval(`cat Cargo.toml | yq -p toml '.features | keys' -o json`)
list.delete("default")
mutually_exclusive = [
  ["hashbrown", "hashmap", "btreemap"]
]

failing_features=[]

list.size.times { |i| 
  a = list.combination(i).map{ _1}
  for features in a do
    if mutually_exclusive.all? { (_1 & features).size == 1 }
      cmd = "cargo nextest run --no-default-features " + features.flat_map{["--features", _1]}.join(" ")
      exit_code=`#{cmd}`

      if $?.exitstatus != 0 
        failing_features << features
      end
    end
  end
}

puts "Features faillings:"
for failing_feature_set in failing_features do
  p failing_feature_set
end
