fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;

	let toshiba_quan:f64 = 2.0;
	let mac_quan:f64 = 1.0;
	let hp_quan:f64 = 3.0;
	let dell_quan:f64 = 3.0;
	let acer_quan:f64 = 1.0;

	let toshiba_total:f64 = toshiba_quan*toshiba;
	let mac_total:f64 = mac*mac_quan;
	let hp_total:f64 = hp_quan*hp;
	let dell_total:f64 = dell_quan*dell;
	let acer_total:f64 = acer_quan*acer;

	let sum = toshiba_total+hp_total+mac_total+acer_total+dell_total;
	let total_quan = toshiba_quan+mac_quan+hp_quan+acer_quan+dell_quan;
	let avg = sum / total_quan;

	println!("THE SUM IS {}", sum);
	println!("the average is {}", avg);
}