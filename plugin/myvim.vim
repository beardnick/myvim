" Initialize the channel
if !exists('s:myvim_jobid')
	let s:myvim_jobid = 0
endif

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:plugin_name   = 'myvim'
let s:plugin_root   = fnamemodify(resolve(expand('<sfile>:p')), ':h:h')
 
let s:bin = s:plugin_root . '/target/release/' . s:plugin_name

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "myvim: cannot start rpc process"
  elseif -1 == id
    echoerr "myvim: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:myvim_jobid = id 
    call s:configureCommands()

  endif
endfunction

" Initialize RPC
function! s:initRpc()
  if s:myvim_jobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:myvim_jobid
  endif
endfunction

function! s:configureCommands()
  command! -nargs=0 Test :call rpcnotify(s:myvim_jobid, "Test")
endfunction

call s:connect()