use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {

    //设置TCP监听地址
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //incoming()方法返回一个产生TcpStream流序列的迭代器，for循环用于处理每个连接
    for stream in listener.incoming() {
        
        //unwarp用于在错误的情形下结束程序
        let stream = stream.unwrap();

        //有连接时输出下面的信息
        println!("Connection established!");

        //使用stream来调用handle_connection函数
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream) {

    //声明一个用于存放数据的buffer缓冲区，大小为512字节
    let mut buffer = [0; 512];

    //使用缓冲区调用stream.read，从TcpStream中读取数据并存入buffer中
    stream.read(&mut buffer).unwrap();

    //使用字节字符串语法b“”将get的文本转换为字节字符串
    let get = b"GET / HTTP/1.1\r\n";

    //如果buffer中的数据是以get中的字节开头，则执行以下操作
    if buffer.starts_with(get) {
        //读取hello.html文件
        let contents = fs::read_to_string("hello.html").unwrap();

        //使用format！把文件的内容添加为成功响应的消息体
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        //使用as_bytes()方法将response字符串转换为字节，并发送到连接中去
        stream.write(response.as_bytes()).unwrap();

        //flush（）会等待并阻止程序继续运行直到所有字节都被写入连接中
        stream.flush().unwrap();

        //若buffer没有以get中的字节开头，以下代码会对这些请求作出处理
    } else {
        //设置包含404状态码以及“404 NOT FOUND”的原因短语
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        //读取404.html文件
        let contents = fs::read_to_string("404.html").unwrap();
        //使用format！把文件的内容添加为响应的消息体
        let response = format!("{}{}", status_line, contents);

        //使用as_bytes()方法将response字符串转换为字节，并发送到连接中去
        stream.write(response.as_bytes()).unwrap();
        
        //flush（）会等待并阻止程序继续运行直到所有字节都被写入连接中
        stream.flush().unwrap();
    }

    
}
