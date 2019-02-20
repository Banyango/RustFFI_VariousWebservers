var express = require('express');
var addon = require('../native/index.node');
var router = express.Router();

/* GET users listing. */
router.get('/', function(req, res, next) {

  var result = addon.fib(parseInt(req.query.i)).toString();

  var response = {
    'result':result
  }

  res.send(JSON.stringify(response));

  
});

module.exports = router;
