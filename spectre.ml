let measure_access_time password guess index =
  let start_time = Sys.time () in
  (* Speculative execution *)
  if String.get password index = guess then
    let _ =
      let temp = ref 0 in
      for _ = 1 to 1000 do
        temp := !temp + 1
      done
    in
    Sys.time () -. start_time
  else
    Sys.time () -. start_time

let crack_password password =
  let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~" in
  let cracked_password = Array.make (String.length password) '*' in
  for i = 0 to String.length password - 1 do
    let max_time = ref 0.0 in
    let max_char = ref '*' in
    String.iter (fun c ->
      let total_time = ref 0.0 in
      for _ = 1 to 100 do
        let access_time = measure_access_time password c i in
        total_time := !total_time +. access_time
      done;
      let avg_time = !total_time /. 100.0 in
      if avg_time > !max_time then (
        max_time := avg_time;
        max_char := c
      )
    ) characters;
    cracked_password.(i) <- !max_char
  done;
  String.init (Array.length cracked_password) (fun i -> cracked_password.(i))

let () =
  let password = "hTQU?K4asW.xl,T\\Gm?vj48`}" in
  Printf.printf "Original Password: %s\n" password;
  let cracked_password = crack_password password in
  Printf.printf "Cracked Password: %s\n" cracked_password
