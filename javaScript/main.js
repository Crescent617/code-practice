const myWorker = new Worker('worker.js');

do_something = function() {
  myWorker.postMessage([10, 10]);
  console.log('Message posted to worker');
}

myWorker.onmessage = function(e) {
  console.log(e)
  console.log('Message received from worker');
}

do_something()