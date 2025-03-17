pub fn convert_moves(alg: &str) -> String {
    let mut result = Vec::new();

    for move_ in alg.split_whitespace() {
        match move_ {
            "M" => {
                result.push("R L' x'");
            }
            "M'" => {
                result.push("R' L x");
            }
            "M2" => {
                result.push("R2 L2 x2");
            }
            "S" => {
                result.push("F' B z");
            }
            "S'" => {
                result.push("F B' z'");
            }
            "S2" => {
                result.push("F2 B2 z2");
            }
            "E" => {
                result.push("U D' y'");
            }
            "E'" => {
                result.push("U' D y");
            }
            "E2" => {
                result.push("U2 D2 y2");
            }

            "u" => {
                result.push("D y");
            }
            "u'" => {
                result.push("D' y'");
            }
            "u2" => {
                result.push("D2 y2");
            }
            "d" => {
                result.push("U y'");
            }
            "d'" => {
                result.push("U' y");
            }
            "d2" => {
                result.push("U2 y2");
            }
            "l" => {
                result.push("R x'");
            }
            "l'" => {
                result.push("R' x");
            }
            "l2" => {
                result.push("R2 x2");
            }
            "r" => {
                result.push("L x");
            }
            "r'" => {
                result.push("L' x'");
            }
            "r2" => {
                result.push("L2 x2");
            }
            "f" => {
                result.push("B z");
            }
            "f'" => {
                result.push("B' z'");
            }
            "f2" => {
                result.push("B2 z2");
            }
            "b" => {
                result.push("F z'");
            }
            "b'" => {
                result.push("F' z");
            }
            "b2" => {
                result.push("F2 z2");
            }

            _ => {
                result.push(move_);
            }
        }
    }

    result.join(" ")
}
