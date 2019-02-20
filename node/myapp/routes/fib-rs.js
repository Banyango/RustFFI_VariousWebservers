var express = require('express');
var ffi = require('ffi');
var router = express.Router();

var testTools = ffi.Library('../../rust/target/release/libffitest.dylib', {
  'fib':['string', ['int']]
})

/* GET users listing. */
router.get('/', function(req, res, next) {

  var result = testTools.fib(req.query.i);

  var response = {
    'result':result
  }

  res.send(JSON.stringify(response));
});

module.exports = router;
