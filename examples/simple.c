struct S1 {
    int a[2][3][4][5];
    int b;
    void**** ptr;
    double c;
    void (*funcptr)(int, double, int[2][3]);
};

enum E1 { a, b, c };

int testfunc(struct S1 s, int a[2][3][3], enum E1 t){
    return 1;
}
struct S1 t;
extern struct S1* s;