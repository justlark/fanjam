#!/usr/bin/env nu

def main [parent_csv: string, parent_column: string, child_csv: string, child_column: string] {
  let parent_data = open $parent_csv
  let child_data = open $child_csv

  let by_id = $child_data | enumerate | each {|x| { value: ($x.item | get $child_column), id: $x.index } }

  $parent_data | update $parent_column {|x| $by_id | where "value" == ($x | get $parent_column) | get "id" | first} | to csv
}
