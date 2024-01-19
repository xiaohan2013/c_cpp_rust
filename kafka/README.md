### 安装librdkakfa
```sh
    git clong git@github.com:confluentinc/librdkafka.git
    git checkout v2.3.0 -b v2.3.0
    ./configure
    make -j2 
    make && make install
```

### 编译代码
```sh
pkg-config --cflags --libs rdkafka
gcc -I/usr/local/include/librdkafka -L/usr/local/lib -Wall -v -g -o producer producer.c -lrdkafka -lm -lcurl -lsasl2 -lssl -lcrypto -lz -ldl -lpthread -lrt

export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib/
./producer 192.168.0.143:19092 test

gcc -I/usr/local/include/librdkafka -L/usr/local/lib -Wall -v -g -o consumer consumer.c -lrdkafka -lm -lcurl -lsasl2 -lssl -lcrypto -lz -ldl -lpthread -lrt

export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib/
./consumer 192.168.0.143:19092 test
```

### C++
```sh

producer_cpp: ../src-cpp/librdkafka++.a ../src/librdkafka.a producer.cpp
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) producer.cpp -o $@ $(LDFLAGS) \
		../src-cpp/librdkafka++.a ../src/librdkafka.a $(LIBS)
    g++ -g -O2 -fPIC -Wall -Wsign-compare -Wfloat-equal -Wpointer-arith -Wcast-align  

g++ -g -O2 -fPIC -Wall -Wsign-compare -Wfloat-equal -Wpointer-arith -Wcast-align -Wno-non-virtual-dtor -I../src-cpp rdkafka_example.cpp -o rdkafka_example_cpp ../src-cpp/librdkafka++.a ../src/librdkafka.a -lm -lcurl -lsasl2 -lssl -lcrypto -lz -ldl -lpthread -lrt

g++ -I/usr/local/include/librdkafka -L/usr/local/lib -g -O2 -fPIC -Wall -v -Wsign-compare -Wfloat-equal -Wpointer-arith -Wcast-align -Wno-non-virtual-dtor producer.cpp -o producer_cpp -lrdkafka++ -lrdkafka -lm -lcurl -lsasl2 -lssl -lcrypto -lz -ldl -lpthread -lrt

export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib/
./producer_cpp 192.168.0.143:19092 test
```