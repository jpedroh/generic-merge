public class Test {
    long fibo(int n) {
        if (n < 2) {
            return n;
        } else {
            return fibo(n - 1) + fibo(n - 2);
        }
    }

	int calc(int a,int b){
		return a+b;
	}
}