use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::io;

pub fn write( s:String ){
    // 書き込み先
    let mut log_file = File::create(Path::new("warizan.txt")).unwrap();
        print!( "{}", s );
    match log_file.write_all( s.as_bytes() )
    {
        Err(_why) => {},
        Ok(_) => {},
    }
}
pub fn writeln( s:String ){
    // 書き込み先
    let mut log_file = File::create(Path::new("warizan.txt")).unwrap();
        println!( "{}", s );
    match log_file.write_all( format!("{}\n", s).as_bytes() )
    {
        Err(_why) => {},
        Ok(_) => {},
    }
}

/**
 * 割られる数を表示
 */
fn hyoji_vec_warareru_su( vec_wararerusu:&Vec<i64> ){

    write( "割られる数の推移： ".to_string() );
    //writeln( format!("vec_wararerusu.len()={}", vec_wararerusu.len() ) );

    for warareru_su in vec_wararerusu.iter() {
        write( format!("{},", warareru_su ) );
    }

    writeln( "".to_string() );
}

/**
 * 循環小数判断機
 */
fn check_recurring( vec_wararerusu:&Vec<i64> )->bool{

    let len = vec_wararerusu.len();

    if len < 2 {
        // 0桁、1桁なら循環してないぜ☆（＾～＾）
        return false;
    }

    // どの繰り返しの時点で　循環小数と考えていいのかだぜ☆（＾～＾）？
    // 長さ1個分の循環小数から考えようぜ☆（＾～＾）
    for nagasa in 1..1000 {
        // 小数点めんどくさいので考えないぜ☆（＾～＾）
        if len <= nagasa*2 {
            break;
        }

        let a_start = len-2*nagasa;
        let b_start = len-nagasa;
        
        let mut itti : bool = true; //一致
        for i_keta in 0..nagasa {
            //writeln( format!("nagasa={} a={} b={} a={} b={}", nagasa, a_start+i_keta, b_start+i_keta, vec_wararerusu[a_start+i_keta], vec_wararerusu[b_start+i_keta] ) );
            if vec_wararerusu[a_start+i_keta]!=vec_wararerusu[b_start+i_keta] {
                itti = false;
                break;
            }
        }

        if itti {
            return true;
        }
    }

    return false;
}

fn main() {

    loop{
        writeln( "さあ、割り算をしようか☆（＾～＾）！
a÷b の a と b に入る 正の整数 を半角空白1個で区切って入れろだぜ☆（＾～＾）
終わりたいときは quit と打ちこめだぜ☆
例： 1 19".to_string() );

        let mut line : String = String::new();
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line : String = line.trim().parse().ok().expect("info Failed to parse");

        if line == "quit" {
            break;
        }

        let vec: Vec<&str> = line.split(" ").collect();
        writeln( format!("{}÷{}☆（＾～＾）！", vec[0], vec[1] ) );


        let mut kekka : String = "".to_string();
        let wararerusu0 : i64 = vec[0].parse().unwrap();
        let warusu0 : i64 = vec[1].parse().unwrap();
        writeln( format!("さあ、割り算をしようか☆（＾～＾）！\n{}÷{} を考えろだぜ☆（＾～＾）！\n", wararerusu0, warusu0 ) );

        let mut vec_wararerusu : Vec<i64> = Vec::new();
        let mut wararerusu : i64 = wararerusu0; // 割られる数
        let warusu : i64 = warusu0; // 割る数
        let mut i_keta : i64 = 0; // 桁
        loop {

            if 256 < i_keta { // 打ち止め
                writeln( format!("({}){} めんどくせ……、やーめた☆（＾～＾）", i_keta, kekka) );

                kekka += &" 計算打ち止め".to_string();
                break;

            } else if wararerusu == 0 {
                writeln( format!("({}){} 割り切れた☆（＾～＾）", i_keta, kekka) );

                kekka += &" 割り切り".to_string();
                break;

            }
            
            if wararerusu < warusu {
                let old_warareru_su = wararerusu;
                wararerusu *= 10;
                writeln( format!("({}) {}÷{} ☆（＾～＾）？ 割れね☆（＾～＾） 10倍して　{}÷{} を考えろだぜ☆（＾～＾）！", i_keta, old_warareru_su, warusu, wararerusu, warusu ) );

                if i_keta==0 {
                    kekka += &".".to_string();
                } else {
                    kekka += &"0".to_string();                
                }
                i_keta += 1;

                writeln( format!("{}", kekka ) );
            }

            if warusu <= wararerusu {
                let old_warareru_su = wararerusu;
                let old_waru_su = warusu;

                let mut wattemiru_su = 0;
                while warusu <= wararerusu {
                    wararerusu -= warusu;
                    wattemiru_su += 1;
                }

                // 100 の割る数で 1 割ると、 "01" をつなげる。

                kekka += &wattemiru_su.to_string();
                writeln( format!("({}) {}÷{}は、{}で割れたぜ☆（＾～＾）", i_keta, old_warareru_su, old_waru_su, wattemiru_su ) );

                // 割ったら、次は割れないので１０倍しておくぜ☆（＾～＾）
                wararerusu *= 10;
                i_keta += 1;

                writeln( format!("{}", kekka ) );
            }
            
            vec_wararerusu.push(wararerusu);
            
            if check_recurring( &vec_wararerusu ) {
                writeln( format!("({}){} 循環小数かだぜ☆（＾～＾）", i_keta, kekka) );

                hyoji_vec_warareru_su( &vec_wararerusu );
                break;
            }
        }

        writeln( format!("{:3}/{:3}={}", wararerusu0, warusu0, kekka ) );
    }
}
