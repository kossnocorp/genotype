public class User {
  private Name name;
  private Integer age;

  public User(Name name, Integer age) {
    this.name = name;
    this.age = age;
  }

  public Name getName() {
    return name;
  }

  public void setName(Name name) {
    this.name = name;
  }

  public Integer getAge() {
    return age;
  }

  public void setAge(Integer age) {
    this.age = age;
  }
}
