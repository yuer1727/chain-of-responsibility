use std::fmt::Debug;

pub trait ConcreteHandlerBase : Debug{

    fn get_handler(&self) -> Option<&HandleBase>;

    fn process(&self, handle_entity: &PurchaseRequest) -> bool;

    fn handle_chain(&self, handle_entity: &PurchaseRequest){
        //无下一级处理
        if !self.process(handle_entity) {
            return;
        }
        match self.get_handler() {
            Some(handler) => handler.handle_next(handle_entity),
            None => {},
        }
    }

}



#[derive(Debug)]
pub struct HandleBase {
    next: Option<Box<dyn ConcreteHandlerBase>>,

}

impl HandleBase {

    fn handle_next(&self, handle_entity: &PurchaseRequest){
        match &self.next {
            Some(next) => {
                next.handle_chain(handle_entity);
            }
            None => {},
        }
    }
}



pub struct PurchaseRequest{
    pub amount: f64,  //采购金额
    pub number: i64, //采购单编号
    pub purpose: String, //采购目的
}

impl PurchaseRequest{
    fn get_amount(&self) -> f64{
        self.amount
    }
    fn get_number(&self) -> i64{
        self.number
    }
    fn get_purpose(&self) -> &String{
        &self.purpose
    }
}




#[cfg(test)]
mod tests {
    use crate::ConcreteHandlerBase;
    use crate::HandleBase;
    use crate::PurchaseRequest;

    //主任：具体处理者
    #[derive(Debug)]
    struct Director {
        pub name: String,
        pub handle_base: Option<HandleBase>,
    }

    //副董事长：具体处理者
    #[derive(Debug)]
    struct VicePresident{
        pub name: String,
        pub handle_base: Option<HandleBase>,
    }

    //董事长：具体处理者
    #[derive(Debug)]
    struct President{
        pub name: String,
        pub handle_base: Option<HandleBase>,
    }

    //董事会：具体处理者
    #[derive(Debug)]
    struct Congress{
        pub name: String,
        pub handle_base: Option<HandleBase>,
    }


    impl ConcreteHandlerBase for Director{

        fn get_handler(&self) -> Option<&HandleBase>{
            match &self.handle_base {
                Some(handle_base) => Some(&handle_base),
                None => None
            }
        }

        fn process(&self, handle_entity: &PurchaseRequest) -> bool {
            println!("主任:{},审批采购数量:{}，金额:{}元，采购目的:{}", self.name, handle_entity.get_number(), handle_entity.get_amount(), handle_entity.get_purpose());
            if handle_entity.get_amount() > 50000.0 {
                return true;
            }
            false
        }

    }

    impl ConcreteHandlerBase for VicePresident{
        fn get_handler(&self) -> Option<&HandleBase>{
            match &self.handle_base {
                Some(handle_base) => Some(&handle_base),
                None => None
            }
        }
        fn process(&self, handle_entity: &PurchaseRequest) -> bool {
            println!("副董事长:{},审批采购数量:{}，金额:{}元，采购目的:{}", self.name, handle_entity.get_number(), handle_entity.get_amount(), handle_entity.get_purpose());
            if handle_entity.get_amount() > 100000.0 {
                return true;
            }
            false
        }
    }

    impl ConcreteHandlerBase for President{
        fn get_handler(&self) -> Option<&HandleBase>{
            match &self.handle_base {
                Some(handle_base) => Some(&handle_base),
                None => None
            }
        }

        fn process(&self, handle_entity: &PurchaseRequest) -> bool {
            println!("董事长:{},审批采购数量:{}，金额:{}元，采购目的:{}", self.name, handle_entity.get_number(), handle_entity.get_amount(), handle_entity.get_purpose());
            if handle_entity.get_amount() > 500000.0 {
                return true;
            }
            false
        }
    }

    impl ConcreteHandlerBase for Congress{

        fn get_handler(&self) -> Option<&HandleBase>{
            match &self.handle_base {
                Some(handle_base) => Some(&handle_base),
                None => None
            }
        }

        fn process(&self, handle_entity: &PurchaseRequest) -> bool {
            println!("董事会:{},审批采购数量:{}，金额:{}元，采购目的:{}", self.name, handle_entity.get_number(), handle_entity.get_amount(), handle_entity.get_purpose());
            false
        }
    }


    #[test]
    fn test_approver() {


        let congress = Congress{
            name: "cc".to_string(),
            handle_base: None,
        };


        let president = President{
            name: "pp".to_string(),
            handle_base: Some(HandleBase{ next: Some(Box::new(congress))}),
        };


        let vicePresident = VicePresident{
            name: "vv".to_string(),
            handle_base: Some(HandleBase{ next: Some(Box::new(president))}),
        };

        let director = Director{
            name: "dd".to_string(),
            handle_base: Some(HandleBase{ next: Some(Box::new(vicePresident))}),
        };

        let pr = PurchaseRequest{ amount: 55000.0, number: 10001, purpose: "项目需要".to_string() };
        let pr2 = PurchaseRequest{ amount: 200000.0, number: 10002, purpose: "饭堂开销需要".to_string() };
        let pr3 = PurchaseRequest{ amount: 2000000.0, number: 10003, purpose: "买办公室".to_string() };

        director.handle_chain(&pr);
        director.handle_chain(&pr2);
        director.handle_chain(&pr3);

    }
}


