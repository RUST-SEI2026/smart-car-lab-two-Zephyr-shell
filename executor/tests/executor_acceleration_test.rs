use executor::{Executor,Pose};

mod acceleration_move_tests{
    use super::*;
    #[test]
    fn should_ruturn_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BM");

        let expected_pose = Pose::new(-1,0,'E');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_plus_2_given_status_is_acceleration_command_is_m_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FM");

        let expected_pose = Pose::new(2,0,'E');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_minus_2_given_status_is_reverse_and_acceleration_command_is_m_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFM");

        let expected_pose = Pose::new(-2,0,'E');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_y_plus_1_given_status_is_reverse_and_reverse_command_is_m_and_facing_is_n(){
        let original_pose = Pose::new(0,0,'N');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BBM");

        let expected_pose = Pose::new(0,1,'N');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_y_plus_1_given_status_is_acceleration_and_acceleration_command_is_m_and_facing_is_n(){
        let original_pose = Pose::new(0,0,'N');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FFM");

        let expected_pose = Pose::new(0,1,'N');
        assert_eq!(expected_pose,executor.query());
    }
}

mod acceleration_turn_left_tests{
    use super::*;
    #[test]
    fn should_ruturn_facing_is_s_given_status_is_reverse_command_is_l_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BL");

        let expected_pose = Pose::new(0,0,'S');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_plus_1_and_facing_is_n_given_status_is_acceleration_command_is_l_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FL");

        let expected_pose = Pose::new(1,0,'N');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_minus_1_and_facing_is_s_given_status_is_acceleration_and_reverse_command_is_l_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFL");

        let expected_pose = Pose::new(-1,0,'S');
        assert_eq!(expected_pose,executor.query());
    }
}

mod acceleration_turn_right_tests{
    use super::*;
    #[test]
    fn should_ruturn_facing_is_n_given_status_is_reverse_command_is_r_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BR");

        let expected_pose = Pose::new(0,0,'N');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_plus_1_and_facing_is_s_given_status_is_acceleraton_command_is_r_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FR");

        let expected_pose = Pose::new(1,0,'S');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_ruturn_x_minus_1_and_facing_is_n_given_status_is_reverse_and_acceleration_command_is_r_and_facing_is_e(){
        let original_pose = Pose::new(0,0,'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFR");

        let expected_pose = Pose::new(-1,0,'N');
        assert_eq!(expected_pose,executor.query());
    }

}