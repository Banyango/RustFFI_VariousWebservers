package com.ffitest.ffitest;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.math.BigInteger;
import java.util.Map;

@RestController
@SpringBootApplication
public class FfitestApplication {

	@RequestMapping(value = "/fib")
	public String fibJava(@RequestParam("i") int value) {
		
		long temp = 0;
		long a = 1;
		long b = 0;

		while(value >= 0) {
			temp = a;
			a = a + b;
			b = temp;
			value--;
		}

		return "{\"result\":"+ b + "}";
	}

	@RequestMapping(value = "/fib-opt")
	public String fibJNI(@RequestParam("i") int value) {
		String result = Fibonnacci.fib(value);

		return "{\"result\":"+ result + "}";
	}

	public static void main(String[] args) {

		String result = Fibonnacci.fib(0);

		SpringApplication.run(FfitestApplication.class, args);
	}

}
