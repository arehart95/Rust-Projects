struct Employee {
    emp_id: u32,
    basic_salary: u32,
    pf: u32,
    med: u32,
    sales: u32
}

impl Employee {
    fn calc_bonus(&self) -> u32 {
        let mut bonus: u32 = 0;
        if self.basic_salary <= 7000 && self.sales < 10 {
            bonus = 1000;
        } else if self.basic_salary <= 7000 && self.sales >= 10 {
            bonus = 1500;
        } else if self.basic_salary > 7000 && self.basic_salary <= 15000 && self.sales < 10 {
            bonus = 2000;
        } else if self.basic_salary > 7000 && self.basic_salary <= 15000 && self.sales >= 10 {
            bonus = 4000;
        } else if self.basic_salary > 15000 && self.sales < 10 {
            bonus = 2500;
        } else if self.basic_salary > 15000 && self.sales >= 10 {
            bonus = 4500;
        }
        bonus
    }

    fn calc_salary(&self) -> u32 {
        let net_salary = self.basic_salary - self.pf - self.med;
        net_salary
    }
}

fn main() {
    let emp = Employee {
        emp_id: 4333,
        basic_salary: 9000,
        pf: 450,
        med: 250,
        sales: 7
    };

    if emp.emp_id == 0 || emp.basic_salary == 0 || emp.pf == 0 {
        println!("Unable to calculate salary.");
    } else {
        let bonus = emp.calc_bonus();
        let net_salary = emp.calc_salary();
        let total_salary = net_salary + bonus;
        println!("Total salary with bonus is {}", total_salary);
    }
}
