var express = require('express');
var router = express.Router();

function fibonacci(num) {
  var a = 1, b = 0, temp;

  while (num >= 0){
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  return b;
}

/* GET users listing. */
router.get('/', function(req, res, next) {

  var result = fibonacci(req.query.i).toString();

  var response = {
    'result':result
  }

  res.send(JSON.stringify(response));

  
});


module.exports = router;
