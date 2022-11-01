pub fn array_vector() {
        //array olustururken size vermek zorundasin,
        //vector icin boyle bir zorunluluk yok ama vektor generic olarak reference aliyor buna dikkat et.

        let points: [i8; 5] = [1, 3, 4, 5, 6];
        for p in points.iter() {
            if *p != (points.len() - 1) as i8 {
                print!("{},", p);
                continue;
            }

            print!("{}\n", p);
        }

        //default da her degisken immutable!

        let mut colors: Vec<&str> = vec!["mavi", "kırmızı", "beyaz", "gri", "sarı"];
        colors.push("pink");
        for c in colors.iter() {
            print!("{}\n", c);
        }
    }